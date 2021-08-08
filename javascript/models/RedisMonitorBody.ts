/* tslint:disable */
/* eslint-disable */
/**
 * Schnooty API
 * This is the Schnooty API. It is used by both our agent and web application to manage your monitors, agents, alerts, account settings and everything else
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@schnooty.com
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { exists, mapValues } from '../runtime';
import {
    BoolExpr,
    BoolExprFromJSON,
    BoolExprFromJSONTyped,
    BoolExprToJSON,
} from './';

/**
 * 
 * @export
 * @interface RedisMonitorBody
 */
export interface RedisMonitorBody {
    /**
     * 
     * @type {string}
     * @memberof RedisMonitorBody
     */
    hostname?: string;
    /**
     * 
     * @type {number}
     * @memberof RedisMonitorBody
     */
    port?: number;
    /**
     * 
     * @type {number}
     * @memberof RedisMonitorBody
     */
    db?: number;
    /**
     * 
     * @type {string}
     * @memberof RedisMonitorBody
     */
    username?: string;
    /**
     * 
     * @type {string}
     * @memberof RedisMonitorBody
     */
    password?: string;
    /**
     * 
     * @type {BoolExpr}
     * @memberof RedisMonitorBody
     */
    expression?: BoolExpr;
}

export function RedisMonitorBodyFromJSON(json: any): RedisMonitorBody {
    return RedisMonitorBodyFromJSONTyped(json, false);
}

export function RedisMonitorBodyFromJSONTyped(json: any, ignoreDiscriminator: boolean): RedisMonitorBody {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'hostname': !exists(json, 'hostname') ? undefined : json['hostname'],
        'port': !exists(json, 'port') ? undefined : json['port'],
        'db': !exists(json, 'db') ? undefined : json['db'],
        'username': !exists(json, 'username') ? undefined : json['username'],
        'password': !exists(json, 'password') ? undefined : json['password'],
        'expression': !exists(json, 'expression') ? undefined : BoolExprFromJSON(json['expression']),
    };
}

export function RedisMonitorBodyToJSON(value?: RedisMonitorBody | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'hostname': value.hostname,
        'port': value.port,
        'db': value.db,
        'username': value.username,
        'password': value.password,
        'expression': BoolExprToJSON(value.expression),
    };
}


