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
/**
 * 
 * @export
 * @interface AgentSessionRequest
 */
export interface AgentSessionRequest {
    /**
     * 
     * @type {string}
     * @memberof AgentSessionRequest
     */
    sessionId: string;
    /**
     * Optional indicator that tells the server that this is a new session.
     * @type {boolean}
     * @memberof AgentSessionRequest
     */
    isNew?: boolean;
}

export function AgentSessionRequestFromJSON(json: any): AgentSessionRequest {
    return AgentSessionRequestFromJSONTyped(json, false);
}

export function AgentSessionRequestFromJSONTyped(json: any, ignoreDiscriminator: boolean): AgentSessionRequest {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'sessionId': json['sessionId'],
        'isNew': !exists(json, 'isNew') ? undefined : json['isNew'],
    };
}

export function AgentSessionRequestToJSON(value?: AgentSessionRequest | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'sessionId': value.sessionId,
        'isNew': value.isNew,
    };
}


