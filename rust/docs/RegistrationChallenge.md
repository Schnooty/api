# RegistrationChallenge

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | [optional] [readonly] [default to None]
**status** | **String** |  | [optional] [readonly] [default to None]
**solution** | **String** | The solution that will activate the challenge. This can be used to prove you are human. | [optional] [default to None]
**type_** | **String** | What the user has to do to pass the challenge. For now this is just `image`. | [readonly] 
**encoding** | **String** | Since this is a JSON api, the image data has to be encoded in some way. For now image data is encoded in base 64. | [readonly] 
**mime_type** | **String** | Describes how to interpret the `data` field. | [readonly] 
**url** | **String** | The URL for the image, if there is one. | [optional] [readonly] [default to None]
**data** | **String** |  | [optional] [readonly] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


