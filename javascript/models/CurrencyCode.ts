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

/**
 * 
 * @export
 * @enum {string}
 */
export enum CurrencyCode {
    NZD = 'NZD',
    USD = 'USD',
    EUR = 'EUR'
}

export function CurrencyCodeFromJSON(json: any): CurrencyCode {
    return CurrencyCodeFromJSONTyped(json, false);
}

export function CurrencyCodeFromJSONTyped(json: any, ignoreDiscriminator: boolean): CurrencyCode {
    return json as CurrencyCode;
}

export function CurrencyCodeToJSON(value?: CurrencyCode | null): any {
    return value as any;
}
