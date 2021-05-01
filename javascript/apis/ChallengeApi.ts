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
    RegistrationChallenge,
    RegistrationChallengeFromJSON,
    RegistrationChallengeToJSON,
} from '../models';

export interface UpdateChallengeRequest {
    id: string;
}

/**
 * 
 */
export class ChallengeApi extends runtime.BaseAPI {

    /**
     * Create a challenge to prove you are human
     */
    async createChallengeRaw(): Promise<runtime.ApiResponse<RegistrationChallenge>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/challenge`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => RegistrationChallengeFromJSON(jsonValue));
    }

    /**
     * Create a challenge to prove you are human
     */
    async createChallenge(): Promise<RegistrationChallenge> {
        const response = await this.createChallengeRaw();
        return await response.value();
    }

    /**
     * Solve a challenge and prove you are human.
     */
    async updateChallengeRaw(requestParameters: UpdateChallengeRequest): Promise<runtime.ApiResponse<RegistrationChallenge>> {
        if (requestParameters.id === null || requestParameters.id === undefined) {
            throw new runtime.RequiredError('id','Required parameter requestParameters.id was null or undefined when calling updateChallenge.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/challenge/{id}`.replace(`{${"id"}}`, encodeURIComponent(String(requestParameters.id))),
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => RegistrationChallengeFromJSON(jsonValue));
    }

    /**
     * Solve a challenge and prove you are human.
     */
    async updateChallenge(requestParameters: UpdateChallengeRequest): Promise<RegistrationChallenge> {
        const response = await this.updateChallengeRaw(requestParameters);
        return await response.value();
    }

}
