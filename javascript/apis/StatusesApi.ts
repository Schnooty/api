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
    MonitorStatusArray,
    MonitorStatusArrayFromJSON,
    MonitorStatusArrayToJSON,
} from '../models';

export interface UpdateMonitorStatusesRequest {
    monitorStatusArray: MonitorStatusArray;
}

/**
 * 
 */
export class StatusesApi extends runtime.BaseAPI {

    /**
     */
    async getMonitorStatusesRaw(): Promise<runtime.ApiResponse<MonitorStatusArray>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = typeof token === 'function' ? token("BearerAuth", []) : token;

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }
        const response = await this.request({
            path: `/statuses`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => MonitorStatusArrayFromJSON(jsonValue));
    }

    /**
     */
    async getMonitorStatuses(): Promise<MonitorStatusArray> {
        const response = await this.getMonitorStatusesRaw();
        return await response.value();
    }

    /**
     */
    async updateMonitorStatusesRaw(requestParameters: UpdateMonitorStatusesRequest): Promise<runtime.ApiResponse<void>> {
        if (requestParameters.monitorStatusArray === null || requestParameters.monitorStatusArray === undefined) {
            throw new runtime.RequiredError('monitorStatusArray','Required parameter requestParameters.monitorStatusArray was null or undefined when calling updateMonitorStatuses.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = typeof token === 'function' ? token("BearerAuth", []) : token;

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }
        const response = await this.request({
            path: `/statuses`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: MonitorStatusArrayToJSON(requestParameters.monitorStatusArray),
        });

        return new runtime.VoidApiResponse(response);
    }

    /**
     */
    async updateMonitorStatuses(requestParameters: UpdateMonitorStatusesRequest): Promise<void> {
        await this.updateMonitorStatusesRaw(requestParameters);
    }

}