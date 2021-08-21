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
    CmpOperator,
    CmpOperatorFromJSON,
    CmpOperatorFromJSONTyped,
    CmpOperatorToJSON,
} from './';

/**
 * 
 * @export
 * @interface FieldConstraint
 */
export interface FieldConstraint {
    /**
     * 
     * @type {string}
     * @memberof FieldConstraint
     */
    name: string;
    /**
     * 
     * @type {CmpOperator}
     * @memberof FieldConstraint
     */
    operator: CmpOperator;
    /**
     * 
     * @type {string}
     * @memberof FieldConstraint
     */
    value: string;
}

export function FieldConstraintFromJSON(json: any): FieldConstraint {
    return FieldConstraintFromJSONTyped(json, false);
}

export function FieldConstraintFromJSONTyped(json: any, ignoreDiscriminator: boolean): FieldConstraint {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'name': json['name'],
        'operator': CmpOperatorFromJSON(json['operator']),
        'value': json['value'],
    };
}

export function FieldConstraintToJSON(value?: FieldConstraint | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'name': value.name,
        'operator': CmpOperatorToJSON(value.operator),
        'value': value.value,
    };
}


