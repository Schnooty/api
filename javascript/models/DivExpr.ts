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
 * @interface DivExpr
 */
export interface DivExpr {
    /**
     * 
     * @type {ArthExpr}
     * @memberof DivExpr
     */
    lhs: ArthExpr;
    /**
     * 
     * @type {ArthExpr}
     * @memberof DivExpr
     */
    rhs: ArthExpr;
    /**
     * 
     * @type {string}
     * @memberof DivExpr
     */
    exprType?: DivExprExprTypeEnum;
}

/**
* @export
* @enum {string}
*/
export enum DivExprExprTypeEnum {
    Div = 'div'
}

export function DivExprFromJSON(json: any): DivExpr {
    return DivExprFromJSONTyped(json, false);
}

export function DivExprFromJSONTyped(json: any, ignoreDiscriminator: boolean): DivExpr {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'lhs': ArthExprFromJSON(json['lhs']),
        'rhs': ArthExprFromJSON(json['rhs']),
        'exprType': !exists(json, 'exprType') ? undefined : json['exprType'],
    };
}

export function DivExprToJSON(value?: DivExpr | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'lhs': ArthExprToJSON(value.lhs),
        'rhs': ArthExprToJSON(value.rhs),
        'exprType': value.exprType,
    };
}


