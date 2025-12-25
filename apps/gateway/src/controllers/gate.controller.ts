/**
 * VeriMantle Gateway - Gate Controller
 * 
 * REST API for guardrails and policy verification.
 * Endpoints:
 * - POST /gate/verify - Verify if an action is allowed
 * - POST /gate/policies - Register a new policy
 * - GET /gate/policies - List all policies
 */

import { Controller, Get, Post, Body, HttpCode, HttpStatus } from '@nestjs/common';
import { GateService } from '../services/gate.service';
import { VerifyActionDto, RegisterPolicyDto } from '../dto/gate.dto';

@Controller('gate')
export class GateController {
  constructor(private readonly gateService: GateService) {}

  @Post('verify')
  @HttpCode(HttpStatus.OK)
  async verify(@Body() dto: VerifyActionDto) {
    return this.gateService.verify(dto.agentId, dto.action, dto.context);
  }

  @Post('policies')
  @HttpCode(HttpStatus.CREATED)
  async registerPolicy(@Body() dto: RegisterPolicyDto) {
    return this.gateService.registerPolicy(dto);
  }

  @Get('policies')
  async getPolicies() {
    return this.gateService.getPolicies();
  }
}
