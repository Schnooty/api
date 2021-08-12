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
    Agent,
    AgentFromJSON,
    AgentFromJSONTyped,
    AgentToJSON,
} from './';

/**
 * 
 * @export
 * @interface NewAgent
 */
export interface NewAgent {
    /**
     * 
     * @type {string}
     * @memberof NewAgent
     */
    readonly apiKey: string;
    /**
     * 
     * @type {string}
     * @memberof NewAgent
     */
    readonly id?: string;
    /**
     * 
     * @type {string}
     * @memberof NewAgent
     */
    name: string;
    /**
     * Describes what this agent is or where it will run.
     * @type {string}
     * @memberof NewAgent
     */
    description?: string;
    /**
     * 
     * @type {string}
     * @memberof NewAgent
     */
    group?: string;
}

export function NewAgentFromJSON(json: any): NewAgent {
    return NewAgentFromJSONTyped(json, false);
}

export function NewAgentFromJSONTyped(json: any, ignoreDiscriminator: boolean): NewAgent {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'apiKey': json['apiKey'],
        'id': !exists(json, 'id') ? undefined : json['id'],
        'name': json['name'],
        'description': !exists(json, 'description') ? undefined : json['description'],
        'group': !exists(json, 'group') ? undefined : json['group'],
    };
}

export function NewAgentToJSON(value?: NewAgent | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'name': value.name,
        'description': value.description,
        'group': value.group,
    };
}


