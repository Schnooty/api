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
    Money,
    MoneyFromJSON,
    MoneyFromJSONTyped,
    MoneyToJSON,
    ProductEntity,
    ProductEntityFromJSON,
    ProductEntityFromJSONTyped,
    ProductEntityToJSON,
} from './';

/**
 * 
 * @export
 * @interface Product
 */
export interface Product {
    /**
     * 
     * @type {ProductEntity}
     * @memberof Product
     */
    entity: ProductEntity;
    /**
     * 
     * @type {Money}
     * @memberof Product
     */
    price: Money;
    /**
     * 
     * @type {number}
     * @memberof Product
     */
    count: number;
    /**
     * 15 monitors for free.
     * @type {string}
     * @memberof Product
     */
    description: string;
}

export function ProductFromJSON(json: any): Product {
    return ProductFromJSONTyped(json, false);
}

export function ProductFromJSONTyped(json: any, ignoreDiscriminator: boolean): Product {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'entity': ProductEntityFromJSON(json['entity']),
        'price': MoneyFromJSON(json['price']),
        'count': json['count'],
        'description': json['description'],
    };
}

export function ProductToJSON(value?: Product | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'entity': ProductEntityToJSON(value.entity),
        'price': MoneyToJSON(value.price),
        'count': value.count,
        'description': value.description,
    };
}


