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

