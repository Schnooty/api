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


import * as runtime from '../runtime';
import {
    ErrorList,
    ErrorListFromJSON,
    ErrorListToJSON,
    ServerInfo,
    ServerInfoFromJSON,
    ServerInfoToJSON,
} from '../models';

/**
 * 
 */
export class InfoApi extends runtime.BaseAPI {

    /**
     */
    async getInfoRaw(): Promise<runtime.ApiResponse<ServerInfo>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/info`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => ServerInfoFromJSON(jsonValue));
    }

    /**
     */
    async getInfo(): Promise<ServerInfo> {
        const response = await this.getInfoRaw();
        return await response.value();
    }

}
