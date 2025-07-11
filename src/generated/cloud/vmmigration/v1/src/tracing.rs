// Copyright 2025 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Code generated by sidekick. DO NOT EDIT.
use crate::Result;

/// Implements a [VmMigration](super::stub::VmMigration) decorator for logging and tracing.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[derive(Clone, Debug)]
pub struct VmMigration<T>
where T: super::stub::VmMigration + std::fmt::Debug + Send + Sync {
    inner: T,
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[derive(Clone, Debug)]
pub struct VmMigration<T>
where T: super::stub::VmMigration + std::fmt::Debug {
    inner: T,
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl<T> VmMigration<T>
where T: super::stub::VmMigration + std::fmt::Debug + Send + Sync {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl<T> VmMigration<T>
where T: super::stub::VmMigration + std::fmt::Debug {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl<T> super::stub::VmMigration for VmMigration<T>
where T: super::stub::VmMigration + std::fmt::Debug + Send + Sync {
    #[tracing::instrument(ret)]
    async fn list_sources(
        &self,
        req: crate::model::ListSourcesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListSourcesResponse>> {
        self.inner.list_sources(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_source(
        &self,
        req: crate::model::GetSourceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Source>> {
        self.inner.get_source(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_source(
        &self,
        req: crate::model::CreateSourceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_source(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_source(
        &self,
        req: crate::model::UpdateSourceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.update_source(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_source(
        &self,
        req: crate::model::DeleteSourceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_source(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn fetch_inventory(
        &self,
        req: crate::model::FetchInventoryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::FetchInventoryResponse>> {
        self.inner.fetch_inventory(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_utilization_reports(
        &self,
        req: crate::model::ListUtilizationReportsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListUtilizationReportsResponse>> {
        self.inner.list_utilization_reports(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_utilization_report(
        &self,
        req: crate::model::GetUtilizationReportRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::UtilizationReport>> {
        self.inner.get_utilization_report(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_utilization_report(
        &self,
        req: crate::model::CreateUtilizationReportRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_utilization_report(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_utilization_report(
        &self,
        req: crate::model::DeleteUtilizationReportRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_utilization_report(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_datacenter_connectors(
        &self,
        req: crate::model::ListDatacenterConnectorsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListDatacenterConnectorsResponse>> {
        self.inner.list_datacenter_connectors(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_datacenter_connector(
        &self,
        req: crate::model::GetDatacenterConnectorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::DatacenterConnector>> {
        self.inner.get_datacenter_connector(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_datacenter_connector(
        &self,
        req: crate::model::CreateDatacenterConnectorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_datacenter_connector(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_datacenter_connector(
        &self,
        req: crate::model::DeleteDatacenterConnectorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_datacenter_connector(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn upgrade_appliance(
        &self,
        req: crate::model::UpgradeApplianceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.upgrade_appliance(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_migrating_vm(
        &self,
        req: crate::model::CreateMigratingVmRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_migrating_vm(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_migrating_vms(
        &self,
        req: crate::model::ListMigratingVmsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListMigratingVmsResponse>> {
        self.inner.list_migrating_vms(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_migrating_vm(
        &self,
        req: crate::model::GetMigratingVmRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::MigratingVm>> {
        self.inner.get_migrating_vm(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_migrating_vm(
        &self,
        req: crate::model::UpdateMigratingVmRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.update_migrating_vm(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_migrating_vm(
        &self,
        req: crate::model::DeleteMigratingVmRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_migrating_vm(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn start_migration(
        &self,
        req: crate::model::StartMigrationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.start_migration(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn resume_migration(
        &self,
        req: crate::model::ResumeMigrationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.resume_migration(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn pause_migration(
        &self,
        req: crate::model::PauseMigrationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.pause_migration(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn finalize_migration(
        &self,
        req: crate::model::FinalizeMigrationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.finalize_migration(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_clone_job(
        &self,
        req: crate::model::CreateCloneJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_clone_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_clone_job(
        &self,
        req: crate::model::CancelCloneJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.cancel_clone_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_clone_jobs(
        &self,
        req: crate::model::ListCloneJobsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListCloneJobsResponse>> {
        self.inner.list_clone_jobs(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_clone_job(
        &self,
        req: crate::model::GetCloneJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::CloneJob>> {
        self.inner.get_clone_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_cutover_job(
        &self,
        req: crate::model::CreateCutoverJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_cutover_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_cutover_job(
        &self,
        req: crate::model::CancelCutoverJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.cancel_cutover_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_cutover_jobs(
        &self,
        req: crate::model::ListCutoverJobsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListCutoverJobsResponse>> {
        self.inner.list_cutover_jobs(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_cutover_job(
        &self,
        req: crate::model::GetCutoverJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::CutoverJob>> {
        self.inner.get_cutover_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_groups(
        &self,
        req: crate::model::ListGroupsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListGroupsResponse>> {
        self.inner.list_groups(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_group(
        &self,
        req: crate::model::GetGroupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Group>> {
        self.inner.get_group(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_group(
        &self,
        req: crate::model::CreateGroupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_group(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_group(
        &self,
        req: crate::model::UpdateGroupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.update_group(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_group(
        &self,
        req: crate::model::DeleteGroupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_group(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn add_group_migration(
        &self,
        req: crate::model::AddGroupMigrationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.add_group_migration(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn remove_group_migration(
        &self,
        req: crate::model::RemoveGroupMigrationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.remove_group_migration(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_target_projects(
        &self,
        req: crate::model::ListTargetProjectsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListTargetProjectsResponse>> {
        self.inner.list_target_projects(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_target_project(
        &self,
        req: crate::model::GetTargetProjectRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::TargetProject>> {
        self.inner.get_target_project(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_target_project(
        &self,
        req: crate::model::CreateTargetProjectRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_target_project(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_target_project(
        &self,
        req: crate::model::UpdateTargetProjectRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.update_target_project(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_target_project(
        &self,
        req: crate::model::DeleteTargetProjectRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_target_project(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_replication_cycles(
        &self,
        req: crate::model::ListReplicationCyclesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListReplicationCyclesResponse>> {
        self.inner.list_replication_cycles(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_replication_cycle(
        &self,
        req: crate::model::GetReplicationCycleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ReplicationCycle>> {
        self.inner.get_replication_cycle(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<location::model::ListLocationsResponse>> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<location::model::Location>> {
        self.inner.get_location(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::ListOperationsResponse>> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.cancel_operation(req, options).await
    }


    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        self.inner.get_polling_error_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl<T> super::stub::VmMigration for VmMigration<T>
where T: super::stub::VmMigration + std::fmt::Debug {
    #[tracing::instrument(ret)]
    async fn list_sources(
        &self,
        req: crate::model::ListSourcesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListSourcesResponse>> {
        self.inner.list_sources(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_source(
        &self,
        req: crate::model::GetSourceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Source>> {
        self.inner.get_source(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_source(
        &self,
        req: crate::model::CreateSourceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_source(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_source(
        &self,
        req: crate::model::UpdateSourceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.update_source(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_source(
        &self,
        req: crate::model::DeleteSourceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_source(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn fetch_inventory(
        &self,
        req: crate::model::FetchInventoryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::FetchInventoryResponse>> {
        self.inner.fetch_inventory(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_utilization_reports(
        &self,
        req: crate::model::ListUtilizationReportsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListUtilizationReportsResponse>> {
        self.inner.list_utilization_reports(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_utilization_report(
        &self,
        req: crate::model::GetUtilizationReportRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::UtilizationReport>> {
        self.inner.get_utilization_report(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_utilization_report(
        &self,
        req: crate::model::CreateUtilizationReportRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_utilization_report(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_utilization_report(
        &self,
        req: crate::model::DeleteUtilizationReportRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_utilization_report(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_datacenter_connectors(
        &self,
        req: crate::model::ListDatacenterConnectorsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListDatacenterConnectorsResponse>> {
        self.inner.list_datacenter_connectors(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_datacenter_connector(
        &self,
        req: crate::model::GetDatacenterConnectorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::DatacenterConnector>> {
        self.inner.get_datacenter_connector(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_datacenter_connector(
        &self,
        req: crate::model::CreateDatacenterConnectorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_datacenter_connector(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_datacenter_connector(
        &self,
        req: crate::model::DeleteDatacenterConnectorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_datacenter_connector(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn upgrade_appliance(
        &self,
        req: crate::model::UpgradeApplianceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.upgrade_appliance(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_migrating_vm(
        &self,
        req: crate::model::CreateMigratingVmRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_migrating_vm(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_migrating_vms(
        &self,
        req: crate::model::ListMigratingVmsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListMigratingVmsResponse>> {
        self.inner.list_migrating_vms(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_migrating_vm(
        &self,
        req: crate::model::GetMigratingVmRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::MigratingVm>> {
        self.inner.get_migrating_vm(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_migrating_vm(
        &self,
        req: crate::model::UpdateMigratingVmRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.update_migrating_vm(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_migrating_vm(
        &self,
        req: crate::model::DeleteMigratingVmRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_migrating_vm(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn start_migration(
        &self,
        req: crate::model::StartMigrationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.start_migration(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn resume_migration(
        &self,
        req: crate::model::ResumeMigrationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.resume_migration(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn pause_migration(
        &self,
        req: crate::model::PauseMigrationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.pause_migration(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn finalize_migration(
        &self,
        req: crate::model::FinalizeMigrationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.finalize_migration(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_clone_job(
        &self,
        req: crate::model::CreateCloneJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_clone_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_clone_job(
        &self,
        req: crate::model::CancelCloneJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.cancel_clone_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_clone_jobs(
        &self,
        req: crate::model::ListCloneJobsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListCloneJobsResponse>> {
        self.inner.list_clone_jobs(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_clone_job(
        &self,
        req: crate::model::GetCloneJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::CloneJob>> {
        self.inner.get_clone_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_cutover_job(
        &self,
        req: crate::model::CreateCutoverJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_cutover_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_cutover_job(
        &self,
        req: crate::model::CancelCutoverJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.cancel_cutover_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_cutover_jobs(
        &self,
        req: crate::model::ListCutoverJobsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListCutoverJobsResponse>> {
        self.inner.list_cutover_jobs(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_cutover_job(
        &self,
        req: crate::model::GetCutoverJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::CutoverJob>> {
        self.inner.get_cutover_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_groups(
        &self,
        req: crate::model::ListGroupsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListGroupsResponse>> {
        self.inner.list_groups(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_group(
        &self,
        req: crate::model::GetGroupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Group>> {
        self.inner.get_group(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_group(
        &self,
        req: crate::model::CreateGroupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_group(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_group(
        &self,
        req: crate::model::UpdateGroupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.update_group(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_group(
        &self,
        req: crate::model::DeleteGroupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_group(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn add_group_migration(
        &self,
        req: crate::model::AddGroupMigrationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.add_group_migration(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn remove_group_migration(
        &self,
        req: crate::model::RemoveGroupMigrationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.remove_group_migration(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_target_projects(
        &self,
        req: crate::model::ListTargetProjectsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListTargetProjectsResponse>> {
        self.inner.list_target_projects(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_target_project(
        &self,
        req: crate::model::GetTargetProjectRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::TargetProject>> {
        self.inner.get_target_project(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_target_project(
        &self,
        req: crate::model::CreateTargetProjectRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_target_project(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_target_project(
        &self,
        req: crate::model::UpdateTargetProjectRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.update_target_project(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_target_project(
        &self,
        req: crate::model::DeleteTargetProjectRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_target_project(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_replication_cycles(
        &self,
        req: crate::model::ListReplicationCyclesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListReplicationCyclesResponse>> {
        self.inner.list_replication_cycles(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_replication_cycle(
        &self,
        req: crate::model::GetReplicationCycleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ReplicationCycle>> {
        self.inner.get_replication_cycle(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<location::model::ListLocationsResponse>> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<location::model::Location>> {
        self.inner.get_location(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::ListOperationsResponse>> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.cancel_operation(req, options).await
    }


    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        self.inner.get_polling_error_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}

