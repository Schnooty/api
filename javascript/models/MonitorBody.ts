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

import {
    HttpMonitorBody,
    ProcessMonitorBody,
    RedisMonitorBody,
    TcpMonitorBody,
    HttpMonitorBodyFromJSONTyped,
    HttpMonitorBodyToJSON,
    ProcessMonitorBodyFromJSONTyped,
    ProcessMonitorBodyToJSON,
    RedisMonitorBodyFromJSONTyped,
    RedisMonitorBodyToJSON,
    TcpMonitorBodyFromJSONTyped,
    TcpMonitorBodyToJSON,
} from './';

/**
 * @type MonitorBody
 * 
 * @export
 */
export type MonitorBody = HttpMonitorBody | ProcessMonitorBody | RedisMonitorBody | TcpMonitorBody;

export function MonitorBodyFromJSON(json: any): MonitorBody {
    return MonitorBodyFromJSONTyped(json, false);
}

export function MonitorBodyFromJSONTyped(json: any, ignoreDiscriminator: boolean): MonitorBody {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return { ...HttpMonitorBodyFromJSONTyped(json, true), ...ProcessMonitorBodyFromJSONTyped(json, true), ...RedisMonitorBodyFromJSONTyped(json, true), ...TcpMonitorBodyFromJSONTyped(json, true) };
}

export function MonitorBodyToJSON(value?: MonitorBody | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return { ...HttpMonitorBodyToJSON(value), ...ProcessMonitorBodyToJSON(value), ...RedisMonitorBodyToJSON(value), ...TcpMonitorBodyToJSON(value) };
}

