/**
 * VeriMantle SDK - Types
 * 
 * Core type definitions for the VeriMantle Agentic Operating System.
 * Per MANIFESTO: Universal Sovereignty - supports any jurisdiction's regulations.
 */

// ============================================================================
// CORE TYPES
// ============================================================================

/**
 * Configuration for initializing the VeriMantle client.
 */
export interface VeriMantleConfig {
  /** API endpoint for VeriMantle services */
  endpoint?: string;
  /** API key for authentication */
  apiKey?: string;
  /** Region for data sovereignty (e.g., 'eu', 'cn', 'sa', 'us') */
  region?: DataResidencyRegion;
  /** Enable debug logging */
  debug?: boolean;
}

/**
 * Data Residency Regions - Per GLOBAL_GAPS.md (Sovereignty Pledge)
 * Supports geo-fenced memory for GDPR, PIPL, and regional compliance.
 */
export type DataResidencyRegion = 
  | 'us'      // United States
  | 'eu'      // European Union (GDPR)
  | 'cn'      // China (PIPL)
  | 'sa'      // Saudi Arabia (Vision 2030)
  | 'in'      // India (DPDP)
  | 'br'      // Brazil (LGPD)
  | 'global'; // Default (no specific residency)

// ============================================================================
// IDENTITY TYPES (VeriMantle-Identity)
// ============================================================================

/**
 * Agent Identity - The "Passport" for autonomous agents.
 */
export interface AgentIdentity {
  /** Unique identifier (DID or UUID) */
  id: string;
  /** Human-readable name */
  name: string;
  /** Public key for signature verification */
  publicKey: string;
  /** Agent capabilities and permissions */
  capabilities: AgentCapability[];
  /** Jurisdiction for legal compliance */
  jurisdiction?: DataResidencyRegion;
  /** Optional wallet address for micropayments */
  walletAddress?: string;
  /** Reputation score (0-100) */
  trustScore?: number;
  /** Creation timestamp */
  createdAt: Date;
}

/**
 * Agent capabilities - defines what actions an agent can perform.
 */
export type AgentCapability = 
  | 'read'
  | 'write'
  | 'execute'
  | 'transfer'
  | 'admin';

/**
 * Signed proof of agent action - for liability tracking.
 */
export interface LiabilityProof {
  /** The action that was performed */
  action: string;
  /** Agent who performed the action */
  agentId: string;
  /** Timestamp of action */
  timestamp: Date;
  /** Cryptographic signature */
  signature: string;
  /** Hash of the payload */
  payloadHash: string;
}

// ============================================================================
// GATE TYPES (VeriMantle-Gate) - Logic & Guardrails
// ============================================================================

/**
 * Policy definition - The "Laws of the Shop" in YAML/DSL format.
 * Per ENGINEERING_STANDARD.md: Neuro-Symbolic Guards.
 */
export interface Policy {
  /** Unique policy identifier */
  id: string;
  /** Human-readable policy name */
  name: string;
  /** Policy rules in DSL format */
  rules: PolicyRule[];
  /** Jurisdictions where this policy applies */
  jurisdictions: DataResidencyRegion[];
  /** Priority (higher = more important) */
  priority: number;
  /** Is this policy active? */
  enabled: boolean;
}

/**
 * Individual policy rule.
 */
export interface PolicyRule {
  /** Rule identifier */
  id: string;
  /** Condition expression (DSL) */
  condition: string;
  /** Action to take if condition matches */
  action: 'allow' | 'deny' | 'review' | 'audit';
  /** Optional message for denials */
  message?: string;
}

/**
 * Result of a policy verification check.
 */
export interface VerificationResult {
  /** Was the action allowed? */
  allowed: boolean;
  /** Policies that were evaluated */
  evaluatedPolicies: string[];
  /** Policies that blocked the action */
  blockingPolicies?: string[];
  /** Risk score (0-100) from Neuro-Symbolic analysis */
  riskScore: number;
  /** Detailed reasoning */
  reasoning?: string;
  /** Latency in milliseconds */
  latencyMs: number;
}

// ============================================================================
// SYNAPSE TYPES (VeriMantle-Synapse) - Memory & State
// ============================================================================

/**
 * Intent Path - Tracks the agent's goal progression.
 * Per ARCHITECTURE.md: Prevents "Intent Drift".
 */
export interface IntentPath {
  /** Unique path identifier */
  id: string;
  /** Original goal/intent */
  originalIntent: string;
  /** Current step in the path */
  currentStep: number;
  /** Total expected steps */
  totalSteps: number;
  /** History of actions taken */
  history: IntentStep[];
  /** Has the agent drifted from original intent? */
  driftDetected: boolean;
  /** Drift score (0-100, higher = more drift) */
  driftScore: number;
}

/**
 * Individual step in an intent path.
 */
export interface IntentStep {
  /** Step number */
  step: number;
  /** Action taken */
  action: string;
  /** Result of the action */
  result?: string;
  /** Timestamp */
  timestamp: Date;
}

/**
 * Agent state stored in Synapse.
 */
export interface AgentState {
  /** Agent identifier */
  agentId: string;
  /** Current intent path */
  intentPath?: IntentPath;
  /** Key-value state storage */
  state: Record<string, unknown>;
  /** Last updated timestamp */
  updatedAt: Date;
  /** Version for CRDT conflict resolution */
  version: number;
}

// ============================================================================
// ARBITER TYPES (VeriMantle-Arbiter) - Coordination & Traffic Control
// ============================================================================

/**
 * Business Lock - Prevents concurrent modifications.
 * Per ARCHITECTURE.md: Atomic Business Locks.
 */
export interface BusinessLock {
  /** Resource being locked */
  resource: string;
  /** Agent holding the lock */
  lockedBy: string;
  /** Lock acquisition time */
  acquiredAt: Date;
  /** Lock expiration time */
  expiresAt: Date;
  /** Lock priority (for conflict resolution) */
  priority: number;
}

/**
 * Coordination request for multi-agent scenarios.
 */
export interface CoordinationRequest {
  /** Requesting agent */
  agentId: string;
  /** Resource to coordinate */
  resource: string;
  /** Type of operation */
  operation: 'read' | 'write' | 'exclusive';
  /** Expected duration in milliseconds */
  expectedDurationMs: number;
  /** Priority level */
  priority: number;
}

/**
 * Result of a coordination request.
 */
export interface CoordinationResult {
  /** Was coordination granted? */
  granted: boolean;
  /** Lock if granted */
  lock?: BusinessLock;
  /** Position in queue if waiting */
  queuePosition?: number;
  /** Estimated wait time in milliseconds */
  estimatedWaitMs?: number;
}
