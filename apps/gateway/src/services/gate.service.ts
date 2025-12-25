/**
 * VeriMantle Gateway - Gate Service
 * 
 * Business logic for policy verification (Guardrails).
 * Per ENGINEERING_STANDARD: Neuro-Symbolic Guards (Code + AI).
 */

import { Injectable } from '@nestjs/common';

interface Policy {
  id: string;
  name: string;
  rules: PolicyRule[];
  jurisdictions: string[];
  priority: number;
  enabled: boolean;
}

interface PolicyRule {
  id: string;
  condition: string;
  action: 'allow' | 'deny' | 'review' | 'audit';
  message?: string;
}

interface VerificationResult {
  allowed: boolean;
  evaluatedPolicies: string[];
  blockingPolicies: string[];
  riskScore: number;
  reasoning?: string;
  latencyMs: number;
}

@Injectable()
export class GateService {
  // In-memory policy store (replace with Redis/database in production)
  private readonly policies = new Map<string, Policy>();

  async verify(
    agentId: string,
    action: string,
    context: Record<string, unknown> = {},
  ): Promise<VerificationResult> {
    const startTime = Date.now();
    const evaluatedPolicies: string[] = [];
    const blockingPolicies: string[] = [];
    let riskScore = 0;

    // Evaluate all enabled policies
    for (const policy of this.policies.values()) {
      if (!policy.enabled) continue;
      
      evaluatedPolicies.push(policy.id);
      
      // Simple rule evaluation (replace with DSL parser in production)
      for (const rule of policy.rules) {
        const isBlocking = this.evaluateRule(rule, action, context);
        if (isBlocking && rule.action === 'deny') {
          blockingPolicies.push(policy.id);
          riskScore = Math.max(riskScore, 100);
        }
      }
    }

    // Calculate risk score based on action type (simplified)
    if (action.includes('delete') || action.includes('remove')) {
      riskScore = Math.max(riskScore, 70);
    } else if (action.includes('transfer') || action.includes('payment')) {
      riskScore = Math.max(riskScore, 50);
    }

    return {
      allowed: blockingPolicies.length === 0,
      evaluatedPolicies,
      blockingPolicies,
      riskScore,
      reasoning: blockingPolicies.length > 0 
        ? `Blocked by policies: ${blockingPolicies.join(', ')}`
        : 'All policies passed',
      latencyMs: Date.now() - startTime,
    };
  }

  async registerPolicy(policy: Policy): Promise<Policy> {
    const policyWithDefaults: Policy = {
      ...policy,
      enabled: policy.enabled ?? true,
    };
    this.policies.set(policy.id, policyWithDefaults);
    return policyWithDefaults;
  }

  async getPolicies(): Promise<Policy[]> {
    return Array.from(this.policies.values());
  }

  private evaluateRule(
    rule: PolicyRule,
    action: string,
    context: Record<string, unknown>,
  ): boolean {
    // Simple condition matching (replace with proper DSL evaluation)
    // Example: "action == 'transfer' && amount > 10000"
    try {
      // For now, just check if action matches a pattern in condition
      if (rule.condition.includes(action)) {
        return true;
      }
      return false;
    } catch {
      return false;
    }
  }
}
