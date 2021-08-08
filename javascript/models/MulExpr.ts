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
 * @interface MulExpr
 */
export interface MulExpr {
    /**
     * 
     * @type {ArthExpr}
     * @memberof MulExpr
     */
    lhs: ArthExpr;
    /**
     * 
     * @type {ArthExpr}
     * @memberof MulExpr
     */
    rhs: ArthExpr;
}

export function MulExprFromJSON(json: any): MulExpr {
    return MulExprFromJSONTyped(json, false);
}

export function MulExprFromJSONTyped(json: any, ignoreDiscriminator: boolean): MulExpr {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'lhs': ArthExprFromJSON(json['lhs']),
        'rhs': ArthExprFromJSON(json['rhs']),
    };
}

export function MulExprToJSON(value?: MulExpr | null): any {
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

