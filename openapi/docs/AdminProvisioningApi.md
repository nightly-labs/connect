# \AdminProvisioningApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_provisioning_reload_dashboards**](AdminProvisioningApi.md#admin_provisioning_reload_dashboards) | **POST** /admin/provisioning/dashboards/reload | Reload dashboard provisioning configurations.
[**admin_provisioning_reload_datasources**](AdminProvisioningApi.md#admin_provisioning_reload_datasources) | **POST** /admin/provisioning/datasources/reload | Reload datasource provisioning configurations.
[**admin_provisioning_reload_plugins**](AdminProvisioningApi.md#admin_provisioning_reload_plugins) | **POST** /admin/provisioning/plugins/reload | Reload plugin provisioning configurations.



## admin_provisioning_reload_dashboards

> models::SuccessResponseBody admin_provisioning_reload_dashboards()
Reload dashboard provisioning configurations.

Reloads the provisioning config files for dashboards again. It won’t return until the new provisioned entities are already stored in the database. In case of dashboards, it will stop polling for changes in dashboard files and then restart it with new configurations after returning. If you are running Grafana Enterprise and have Fine-grained access control enabled, you need to have a permission with action `provisioning:reload` and scope `provisioners:dashboards`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_provisioning_reload_datasources

> models::SuccessResponseBody admin_provisioning_reload_datasources()
Reload datasource provisioning configurations.

Reloads the provisioning config files for datasources again. It won’t return until the new provisioned entities are already stored in the database. In case of dashboards, it will stop polling for changes in dashboard files and then restart it with new configurations after returning. If you are running Grafana Enterprise and have Fine-grained access control enabled, you need to have a permission with action `provisioning:reload` and scope `provisioners:datasources`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_provisioning_reload_plugins

> models::SuccessResponseBody admin_provisioning_reload_plugins()
Reload plugin provisioning configurations.

Reloads the provisioning config files for plugins again. It won’t return until the new provisioned entities are already stored in the database. In case of dashboards, it will stop polling for changes in dashboard files and then restart it with new configurations after returning. If you are running Grafana Enterprise and have Fine-grained access control enabled, you need to have a permission with action `provisioning:reload` and scope `provisioners:plugin`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

