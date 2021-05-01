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
/**
 * 
 * @export
 * @interface HttpHeader
 */
export interface HttpHeader {
    /**
     * 
     * @type {string}
     * @memberof HttpHeader
     */
    name: string;
    /**
     * 
     * @type {string}
     * @memberof HttpHeader
     */
    value: string;
}

export function HttpHeaderFromJSON(json: any): HttpHeader {
    return HttpHeaderFromJSONTyped(json, false);
}

export function HttpHeaderFromJSONTyped(json: any, ignoreDiscriminator: boolean): HttpHeader {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'name': json['name'],
        'value': json['value'],
    };
}

export function HttpHeaderToJSON(value?: HttpHeader | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'name': value.name,
        'value': value.value,
    };
}

