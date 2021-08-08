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
 * @interface ConstExpr
 */
export interface ConstExpr {
    /**
     * 
     * @type {number}
     * @memberof ConstExpr
     */
    constant: number;
}

export function ConstExprFromJSON(json: any): ConstExpr {
    return ConstExprFromJSONTyped(json, false);
}

export function ConstExprFromJSONTyped(json: any, ignoreDiscriminator: boolean): ConstExpr {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'constant': json['constant'],
    };
}

export function ConstExprToJSON(value?: ConstExpr | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'constant': value.constant,
    };
}

