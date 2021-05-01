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
    Account,
    AccountFromJSON,
    AccountToJSON,
    ErrorList,
    ErrorListFromJSON,
    ErrorListToJSON,
    InlineResponse200,
    InlineResponse200FromJSON,
    InlineResponse200ToJSON,
    InlineResponse2001,
    InlineResponse2001FromJSON,
    InlineResponse2001ToJSON,
    InlineResponse2002,
    InlineResponse2002FromJSON,
    InlineResponse2002ToJSON,
    PlanArray,
    PlanArrayFromJSON,
    PlanArrayToJSON,
    Subscription,
    SubscriptionFromJSON,
    SubscriptionToJSON,
    SubscriptionContainer,
    SubscriptionContainerFromJSON,
    SubscriptionContainerToJSON,
    TransactionList,
    TransactionListFromJSON,
    TransactionListToJSON,
} from '../models';

export interface AccountsIdPutRequest {
    id: string;
    account: Account;
}

export interface CreateAccountRequest {
    account: Account;
}

export interface CreateSubscriptionRequest {
    subscription: Subscription;
}

export interface GetAccountBalanceRequest {
    accountId: string;
}

export interface GetAccountByIdRequest {
    id: string;
}

export interface GetSubscriptionByIdRequest {
    id: string;
}

export interface GetTransactionsRequest {
    accountId: string;
    fromTimestamp?: Date;
    toTimestamp?: Date;
}

/**
 * 
 */
export class AccountManagementApi extends runtime.BaseAPI {

    /**
     */
    async accountsIdPutRaw(requestParameters: AccountsIdPutRequest): Promise<runtime.ApiResponse<InlineResponse2001>> {
        if (requestParameters.id === null || requestParameters.id === undefined) {
            throw new runtime.RequiredError('id','Required parameter requestParameters.id was null or undefined when calling accountsIdPut.');
        }

        if (requestParameters.account === null || requestParameters.account === undefined) {
            throw new runtime.RequiredError('account','Required parameter requestParameters.account was null or undefined when calling accountsIdPut.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/accounts/{id}`.replace(`{${"id"}}`, encodeURIComponent(String(requestParameters.id))),
            method: 'PUT',
            headers: headerParameters,
            query: queryParameters,
            body: AccountToJSON(requestParameters.account),
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => InlineResponse2001FromJSON(jsonValue));
    }

    /**
     */
    async accountsIdPut(requestParameters: AccountsIdPutRequest): Promise<InlineResponse2001> {
        const response = await this.accountsIdPutRaw(requestParameters);
        return await response.value();
    }

    /**
     */
    async createAccountRaw(requestParameters: CreateAccountRequest): Promise<runtime.ApiResponse<InlineResponse2001>> {
        if (requestParameters.account === null || requestParameters.account === undefined) {
            throw new runtime.RequiredError('account','Required parameter requestParameters.account was null or undefined when calling createAccount.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/accounts`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: AccountToJSON(requestParameters.account),
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => InlineResponse2001FromJSON(jsonValue));
    }

    /**
     */
    async createAccount(requestParameters: CreateAccountRequest): Promise<InlineResponse2001> {
        const response = await this.createAccountRaw(requestParameters);
        return await response.value();
    }

    /**
     */
    async createSubscriptionRaw(requestParameters: CreateSubscriptionRequest): Promise<runtime.ApiResponse<SubscriptionContainer>> {
        if (requestParameters.subscription === null || requestParameters.subscription === undefined) {
            throw new runtime.RequiredError('subscription','Required parameter requestParameters.subscription was null or undefined when calling createSubscription.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/subscriptions`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: SubscriptionToJSON(requestParameters.subscription),
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => SubscriptionContainerFromJSON(jsonValue));
    }

    /**
     */
    async createSubscription(requestParameters: CreateSubscriptionRequest): Promise<SubscriptionContainer> {
        const response = await this.createSubscriptionRaw(requestParameters);
        return await response.value();
    }

    /**
     */
    async getAccountBalanceRaw(requestParameters: GetAccountBalanceRequest): Promise<runtime.ApiResponse<InlineResponse2002>> {
        if (requestParameters.accountId === null || requestParameters.accountId === undefined) {
            throw new runtime.RequiredError('accountId','Required parameter requestParameters.accountId was null or undefined when calling getAccountBalance.');
        }

        const queryParameters: any = {};

        if (requestParameters.accountId !== undefined) {
            queryParameters['accountId'] = requestParameters.accountId;
        }

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/balances`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => InlineResponse2002FromJSON(jsonValue));
    }

    /**
     */
    async getAccountBalance(requestParameters: GetAccountBalanceRequest): Promise<InlineResponse2002> {
        const response = await this.getAccountBalanceRaw(requestParameters);
        return await response.value();
    }

    /**
     */
    async getAccountByIdRaw(requestParameters: GetAccountByIdRequest): Promise<runtime.ApiResponse<InlineResponse2001>> {
        if (requestParameters.id === null || requestParameters.id === undefined) {
            throw new runtime.RequiredError('id','Required parameter requestParameters.id was null or undefined when calling getAccountById.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/accounts/{id}`.replace(`{${"id"}}`, encodeURIComponent(String(requestParameters.id))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => InlineResponse2001FromJSON(jsonValue));
    }

    /**
     */
    async getAccountById(requestParameters: GetAccountByIdRequest): Promise<InlineResponse2001> {
        const response = await this.getAccountByIdRaw(requestParameters);
        return await response.value();
    }

    /**
     */
    async getAccountsRaw(): Promise<runtime.ApiResponse<InlineResponse200>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/accounts`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => InlineResponse200FromJSON(jsonValue));
    }

    /**
     */
    async getAccounts(): Promise<InlineResponse200> {
        const response = await this.getAccountsRaw();
        return await response.value();
    }

    /**
     */
    async getPlansRaw(): Promise<runtime.ApiResponse<PlanArray>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/plans`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => PlanArrayFromJSON(jsonValue));
    }

    /**
     */
    async getPlans(): Promise<PlanArray> {
        const response = await this.getPlansRaw();
        return await response.value();
    }

    /**
     */
    async getSubscriptionByIdRaw(requestParameters: GetSubscriptionByIdRequest): Promise<runtime.ApiResponse<SubscriptionContainer>> {
        if (requestParameters.id === null || requestParameters.id === undefined) {
            throw new runtime.RequiredError('id','Required parameter requestParameters.id was null or undefined when calling getSubscriptionById.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/subscriptions/{id}`.replace(`{${"id"}}`, encodeURIComponent(String(requestParameters.id))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => SubscriptionContainerFromJSON(jsonValue));
    }

    /**
     */
    async getSubscriptionById(requestParameters: GetSubscriptionByIdRequest): Promise<SubscriptionContainer> {
        const response = await this.getSubscriptionByIdRaw(requestParameters);
        return await response.value();
    }

    /**
     */
    async getSubscriptionRecordsRaw(): Promise<runtime.ApiResponse<Array<Subscription>>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/subscriptions`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => jsonValue.map(SubscriptionFromJSON));
    }

    /**
     */
    async getSubscriptionRecords(): Promise<Array<Subscription>> {
        const response = await this.getSubscriptionRecordsRaw();
        return await response.value();
    }

    /**
     */
    async getTransactionsRaw(requestParameters: GetTransactionsRequest): Promise<runtime.ApiResponse<TransactionList>> {
        if (requestParameters.accountId === null || requestParameters.accountId === undefined) {
            throw new runtime.RequiredError('accountId','Required parameter requestParameters.accountId was null or undefined when calling getTransactions.');
        }

        const queryParameters: any = {};

        if (requestParameters.fromTimestamp !== undefined) {
            queryParameters['fromTimestamp'] = (requestParameters.fromTimestamp as any).toISOString();
        }

        if (requestParameters.toTimestamp !== undefined) {
            queryParameters['toTimestamp'] = (requestParameters.toTimestamp as any).toISOString();
        }

        if (requestParameters.accountId !== undefined) {
            queryParameters['accountId'] = requestParameters.accountId;
        }

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/transactions`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => TransactionListFromJSON(jsonValue));
    }

    /**
     */
    async getTransactions(requestParameters: GetTransactionsRequest): Promise<TransactionList> {
        const response = await this.getTransactionsRaw(requestParameters);
        return await response.value();
    }

}