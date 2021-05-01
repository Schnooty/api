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
    CurrencyCode,
    CurrencyCodeFromJSON,
    CurrencyCodeFromJSONTyped,
    CurrencyCodeToJSON,
} from './';

/**
 * 
 * @export
 * @interface Money
 */
export interface Money {
    /**
     * 
     * @type {number}
     * @memberof Money
     */
    value: number;
    /**
     * 
     * @type {number}
     * @memberof Money
     */
    decimalPlaces: number;
    /**
     * 
     * @type {CurrencyCode}
     * @memberof Money
     */
    currencyCode: CurrencyCode;
}

export function MoneyFromJSON(json: any): Money {
    return MoneyFromJSONTyped(json, false);
}

export function MoneyFromJSONTyped(json: any, ignoreDiscriminator: boolean): Money {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'value': json['value'],
        'decimalPlaces': json['decimalPlaces'],
        'currencyCode': CurrencyCodeFromJSON(json['currencyCode']),
    };
}

export function MoneyToJSON(value?: Money | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'value': value.value,
        'decimalPlaces': value.decimalPlaces,
        'currencyCode': CurrencyCodeToJSON(value.currencyCode),
    };
}

