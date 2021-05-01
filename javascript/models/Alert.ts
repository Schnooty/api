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
import {
    AlertBody,
    AlertBodyFromJSON,
    AlertBodyFromJSONTyped,
    AlertBodyToJSON,
} from './';

/**
 * 
 * @export
 * @interface Alert
 */
export interface Alert {
    /**
     * 
     * @type {string}
     * @memberof Alert
     */
    type: AlertTypeEnum;
    /**
     * 
     * @type {string}
     * @memberof Alert
     */
    readonly id?: string;
    /**
     * Describes what your alert will do.
     * @type {string}
     * @memberof Alert
     */
    description?: string;
    /**
     * 
     * @type {Array<string>}
     * @memberof Alert
     */
    monitors: Array<string>;
    /**
     * 
     * @type {number}
     * @memberof Alert
     */
    threshold: number;
    /**
     * 
     * @type {boolean}
     * @memberof Alert
     */
    enabled: boolean;
    /**
     * 
     * @type {AlertBody}
     * @memberof Alert
     */
    body: AlertBody;
}

export function AlertFromJSON(json: any): Alert {
    return AlertFromJSONTyped(json, false);
}

export function AlertFromJSONTyped(json: any, ignoreDiscriminator: boolean): Alert {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'type': json['type'],
        'id': !exists(json, 'id') ? undefined : json['id'],
        'description': !exists(json, 'description') ? undefined : json['description'],
        'monitors': json['monitors'],
        'threshold': json['threshold'],
        'enabled': json['enabled'],
        'body': AlertBodyFromJSON(json['body']),
    };
}

export function AlertToJSON(value?: Alert | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'type': value.type,
        'description': value.description,
        'monitors': value.monitors,
        'threshold': value.threshold,
        'enabled': value.enabled,
        'body': AlertBodyToJSON(value.body),
    };
}

/**
* @export
* @enum {string}
*/
export enum AlertTypeEnum {
    Msteams = 'msteams',
    Webhook = 'webhook',
    Email = 'email'
}


