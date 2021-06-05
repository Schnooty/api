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
    Money,
    MoneyFromJSON,
    MoneyFromJSONTyped,
    MoneyToJSON,
} from './';

/**
 * 
 * @export
 * @interface Balance
 */
export interface Balance {
    /**
     * 
     * @type {string}
     * @memberof Balance
     */
    readonly accountId: string;
    /**
     * UTC UNIX timestamp in with fractional offset.
     * @type {Date}
     * @memberof Balance
     */
    readonly timestamp: Date;
    /**
     * 
     * @type {Money}
     * @memberof Balance
     */
    balance: Money;
    /**
     * 
     * @type {Money}
     * @memberof Balance
     */
    availableBalance: Money;
}

export function BalanceFromJSON(json: any): Balance {
    return BalanceFromJSONTyped(json, false);
}

export function BalanceFromJSONTyped(json: any, ignoreDiscriminator: boolean): Balance {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'accountId': json['accountId'],
        'timestamp': (new Date(json['timestamp'])),
        'balance': MoneyFromJSON(json['balance']),
        'availableBalance': MoneyFromJSON(json['availableBalance']),
    };
}

export function BalanceToJSON(value?: Balance | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'balance': MoneyToJSON(value.balance),
        'availableBalance': MoneyToJSON(value.availableBalance),
    };
}


