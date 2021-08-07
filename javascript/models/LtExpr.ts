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
    ArthExpr,
    ArthExprFromJSON,
    ArthExprFromJSONTyped,
    ArthExprToJSON,
} from './';

/**
 * 
 * @export
 * @interface LtExpr
 */
export interface LtExpr {
    /**
     * 
     * @type {ArthExpr}
     * @memberof LtExpr
     */
    lhs: ArthExpr;
    /**
     * 
     * @type {ArthExpr}
     * @memberof LtExpr
     */
    rhs: ArthExpr;
}

export function LtExprFromJSON(json: any): LtExpr {
    return LtExprFromJSONTyped(json, false);
}

export function LtExprFromJSONTyped(json: any, ignoreDiscriminator: boolean): LtExpr {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'lhs': ArthExprFromJSON(json['lhs']),
        'rhs': ArthExprFromJSON(json['rhs']),
    };
}

export function LtExprToJSON(value?: LtExpr | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'lhs': ArthExprToJSON(value.lhs),
        'rhs': ArthExprToJSON(value.rhs),
    };
}


