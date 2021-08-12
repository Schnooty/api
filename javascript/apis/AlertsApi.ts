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


import * as runtime from '../runtime';
import {
    Alert,
    AlertFromJSON,
    AlertToJSON,
    AlertArray,
    AlertArrayFromJSON,
    AlertArrayToJSON,
    ErrorList,
    ErrorListFromJSON,
    ErrorListToJSON,
} from '../models';

export interface AlertsIdDeleteRequest {
    id: string;
}

export interface AlertsIdGetRequest {
    id: string;
}

export interface AlertsIdPutRequest {
    id: string;
    alert: Alert;
}

export interface AlertsPostRequest {
    alert: Alert;
}

/**
 * 
 */
export class AlertsApi extends runtime.BaseAPI {

    /**
     * Retrieve all of the alerts you have created in your account.
     */
    async alertsGetRaw(): Promise<runtime.ApiResponse<AlertArray>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }
        const response = await this.request({
            path: `/alerts`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => AlertArrayFromJSON(jsonValue));
    }

    /**
     * Retrieve all of the alerts you have created in your account.
     */
    async alertsGet(): Promise<AlertArray> {
        const response = await this.alertsGetRaw();
        return await response.value();
    }

    /**
     */
    async alertsIdDeleteRaw(requestParameters: AlertsIdDeleteRequest): Promise<runtime.ApiResponse<Alert>> {
        if (requestParameters.id === null || requestParameters.id === undefined) {
            throw new runtime.RequiredError('id','Required parameter requestParameters.id was null or undefined when calling alertsIdDelete.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }
        const response = await this.request({
            path: `/alerts/{id}`.replace(`{${"id"}}`, encodeURIComponent(String(requestParameters.id))),
            method: 'DELETE',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => AlertFromJSON(jsonValue));
    }

    /**
     */
    async alertsIdDelete(requestParameters: AlertsIdDeleteRequest): Promise<Alert> {
        const response = await this.alertsIdDeleteRaw(requestParameters);
        return await response.value();
    }

    /**
     */
    async alertsIdGetRaw(requestParameters: AlertsIdGetRequest): Promise<runtime.ApiResponse<Alert>> {
        if (requestParameters.id === null || requestParameters.id === undefined) {
            throw new runtime.RequiredError('id','Required parameter requestParameters.id was null or undefined when calling alertsIdGet.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }
        const response = await this.request({
            path: `/alerts/{id}`.replace(`{${"id"}}`, encodeURIComponent(String(requestParameters.id))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => AlertFromJSON(jsonValue));
    }

    /**
     */
    async alertsIdGet(requestParameters: AlertsIdGetRequest): Promise<Alert> {
        const response = await this.alertsIdGetRaw(requestParameters);
        return await response.value();
    }

    /**
     */
    async alertsIdPutRaw(requestParameters: AlertsIdPutRequest): Promise<runtime.ApiResponse<Alert>> {
        if (requestParameters.id === null || requestParameters.id === undefined) {
            throw new runtime.RequiredError('id','Required parameter requestParameters.id was null or undefined when calling alertsIdPut.');
        }

        if (requestParameters.alert === null || requestParameters.alert === undefined) {
            throw new runtime.RequiredError('alert','Required parameter requestParameters.alert was null or undefined when calling alertsIdPut.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }
        const response = await this.request({
            path: `/alerts/{id}`.replace(`{${"id"}}`, encodeURIComponent(String(requestParameters.id))),
            method: 'PUT',
            headers: headerParameters,
            query: queryParameters,
            body: AlertToJSON(requestParameters.alert),
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => AlertFromJSON(jsonValue));
    }

    /**
     */
    async alertsIdPut(requestParameters: AlertsIdPutRequest): Promise<Alert> {
        const response = await this.alertsIdPutRaw(requestParameters);
        return await response.value();
    }

    /**
     * Create a new alert that can be activated when a monitor goes down. An alert can be a message (email, Microsoft teams), or it can call a web service (webhook). An alert is associated with zero or more of your monitors. If your alert has more than one monitor, the alert will be activated only when the number of monitors that goes down exceededs the `threshold`.
     */
    async alertsPostRaw(requestParameters: AlertsPostRequest): Promise<runtime.ApiResponse<Alert>> {
        if (requestParameters.alert === null || requestParameters.alert === undefined) {
            throw new runtime.RequiredError('alert','Required parameter requestParameters.alert was null or undefined when calling alertsPost.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/alerts`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: AlertToJSON(requestParameters.alert),
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => AlertFromJSON(jsonValue));
    }

    /**
     * Create a new alert that can be activated when a monitor goes down. An alert can be a message (email, Microsoft teams), or it can call a web service (webhook). An alert is associated with zero or more of your monitors. If your alert has more than one monitor, the alert will be activated only when the number of monitors that goes down exceededs the `threshold`.
     */
    async alertsPost(requestParameters: AlertsPostRequest): Promise<Alert> {
        const response = await this.alertsPostRaw(requestParameters);
        return await response.value();
    }

}
