/* tslint:disable */
/* eslint-disable */
/**
 * Open Monitors
 * This is the Open Monitors API. All operations that a user or an agent would want to complete, including signing up, are listed here.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@openmonitors.com
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { exists, mapValues } from '../runtime';
/**
 * 
 * @export
 * @interface TcpMonitorBody
 */
export interface TcpMonitorBody {
    /**
     * 
     * @type {string}
     * @memberof TcpMonitorBody
     */
    hostname?: string;
    /**
     * 
     * @type {number}
     * @memberof TcpMonitorBody
     */
    port?: number;
}

export function TcpMonitorBodyFromJSON(json: any): TcpMonitorBody {
    return TcpMonitorBodyFromJSONTyped(json, false);
}

export function TcpMonitorBodyFromJSONTyped(json: any, ignoreDiscriminator: boolean): TcpMonitorBody {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'hostname': !exists(json, 'hostname') ? undefined : json['hostname'],
        'port': !exists(json, 'port') ? undefined : json['port'],
    };
}

export function TcpMonitorBodyToJSON(value?: TcpMonitorBody | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'hostname': value.hostname,
        'port': value.port,
    };
}


