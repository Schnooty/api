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
 * @interface FieldExpr
 */
export interface FieldExpr {
    /**
     * 
     * @type {string}
     * @memberof FieldExpr
     */
    name: string;
    /**
     * 
     * @type {string}
     * @memberof FieldExpr
     */
    exprType?: FieldExprExprTypeEnum;
}

/**
* @export
* @enum {string}
*/
export enum FieldExprExprTypeEnum {
    Field = 'field'
}

export function FieldExprFromJSON(json: any): FieldExpr {
    return FieldExprFromJSONTyped(json, false);
}

export function FieldExprFromJSONTyped(json: any, ignoreDiscriminator: boolean): FieldExpr {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'name': json['name'],
        'exprType': !exists(json, 'exprType') ? undefined : json['exprType'],
    };
}

export function FieldExprToJSON(value?: FieldExpr | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'name': value.name,
        'exprType': value.exprType,
    };
}


