# RegistrationChallenge
## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**String**](string.md) |  | [optional] [default to null]
**status** | [**String**](string.md) |  | [optional] [default to null]
**solution** | [**String**](string.md) | The solution that will activate the challenge. This can be used to prove you are human. | [optional] [default to null]
**type** | [**String**](string.md) | What the user has to do to pass the challenge. For now this is just &#x60;image&#x60;. | [default to null]
**encoding** | [**String**](string.md) | Since this is a JSON api, the image data has to be encoded in some way. For now image data is encoded in base 64. | [default to null]
**mimeType** | [**String**](string.md) | Describes how to interpret the &#x60;data&#x60; field. | [default to null]
**url** | [**String**](string.md) | The URL for the image, if there is one. | [optional] [default to null]
**data** | [**String**](string.md) |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

