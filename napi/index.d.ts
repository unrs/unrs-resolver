/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface ResolveResult {
  path?: string
  error?: string
}
export function sync(path: string, request: string): ResolveResult
export class ResolverFactory {
  constructor()
  sync(path: string, request: string): ResolveResult
}
