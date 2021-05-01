# JwtObject

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**jwt** | **String** | The JSON Web Token that can be used to access the API. | 
**jwt_expiry** | **f64** | Time at which the JWT expires in seconds since Unix Epoc | 
**jwt_scopes** | **Vec<String>** |  | 
**refresh_token** | **String** | A Bearer token that can used to get a new JWT. | 
**refresh_token_expiry** | **f64** | Time at which the refresh token expires in seconds since Unix Epoc | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


