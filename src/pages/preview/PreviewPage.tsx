import { useEffect, useMemo } from 'react';
import { useNavigate } from 'react-router-dom';
import { useExecutionStore } from '../../state/executionStore';
import { usePipelineStore } from '../../state/pipelineStore';
import { useTmdbSearchStore } from '../../state/tmdbSearchStore';
import { useUiFeedbackStore } from '../../state/uiFeedbackStore';
import { EditNameModal, usePreviewEditModal } from './edit';
import { PreviewExecutionPanels } from './execution';
import { PreviewHeader } from './header';
import { buildPreviewGroups } from './model/buildPreviewGroups';
import { PreviewNamingWorkbenchSidebar, usePreviewNamingWorkbench } from './naming/workbench';
import { PreviewEmptyState, PreviewLoadingState } from './state';
import { TmdbDisabledBanner, TmdbInspectorSidebar, usePreviewTmdbSearch } from './tmdb';
import { PreviewPlanTreePanel, usePreviewGroupSkip, usePreviewTreeSelection } from './tree';

export function PreviewPage() {
  const navigate = useNavigate();
  const {
    pipelineResult,
    updatePreviewProposedName,
    togglePreviewSkipped,
    refreshSafetySummary,
    applyNamingRule,
    applyFolderPolicy,
    applyTmdbCandidate,
  } = usePipelineStore();
  const { isLoading } = useUiFeedbackStore();
  const { tmdbSearchStatus, disabledReason, searchTmdbCandidates, checkTmdbSearchAvailability } = useTmdbSearchStore();
  const {
    uiState,
    confirmState,
    progressState,
    resultState,
    startExecution,
    confirmExecution,
    cancelExecution,
    resetExecution,
  } = useExecutionStore();
  const {
    selectedPreset,
    currentNamingRule,
    currentStrategy,
    folderPolicyConfig,
    setSelectedPreset,
    setCurrentNamingRule,
    setCurrentStrategy,
    triggerNamingApply,
    handleFolderPolicyChange,
  } = usePreviewNamingWorkbench({
    onApplyNamingRule: applyNamingRule,
    onApplyFolderPolicy: applyFolderPolicy,
  });

  useEffect(() => {
    checkTmdbSearchAvailability();
  }, [checkTmdbSearchAvailability]);

  const scanRoot = pipelineResult?.scan?.scan_path;
  const planTree = useMemo(() => {
    if (!pipelineResult) return null;
    return buildPreviewGroups(pipelineResult.previews, scanRoot);
  }, [pipelineResult, scanRoot]);
  const groups = planTree?.groups ?? [];
  const { expandedGroups, selectedGroup, selectGroup, toggleGroupExpanded } = usePreviewTreeSelection(groups);
  const skipGroup = usePreviewGroupSkip(groups, togglePreviewSkipped);
  const {
    editModal,
    openGroupEdit,
    openFileEdit,
    confirmEdit,
    cancelEdit,
  } = usePreviewEditModal({
    groups,
    previews: pipelineResult?.previews ?? [],
    onNameChange: updatePreviewProposedName,
  });
  const {
    tmdbMediaType,
    tmdbQuery,
    tmdbLoading,
    tmdbError,
    tmdbCandidates,
    selectedCandidate,
    setTmdbMediaType,
    setTmdbQuery,
    setSelectedCandidate,
    handleTmdbSearch,
    handleGroupTmdbSearch,
    handleApplyCandidate,
    clearCandidates,
  } = usePreviewTmdbSearch({
    groups,
    selectedGroup,
    tmdbSearchStatus,
    disabledReason,
    onGroupSelect: selectGroup,
    onSearchCandidates: searchTmdbCandidates,
    onApplyCandidate: applyTmdbCandidate,
    onRefreshSafetySummary: refreshSafetySummary,
  });

  if (isLoading) {
    return <PreviewLoadingState />;
  }

  if (!pipelineResult) {
    return <PreviewEmptyState onScan={() => navigate('/scan')} />;
  }

  return (
    <div className="flex h-[calc(100vh-6rem)] flex-col gap-3">
      <PreviewHeader
        groupCount={groups.length}
        fileCount={pipelineResult.previews.length}
        onBack={() => navigate('/scan')}
        onSettings={() => navigate('/settings')}
        onDryRun={() => startExecution('DryRun')}
        onExecute={() => startExecution('Confirmed')}
      />

      {tmdbSearchStatus === 'disabled' && (
        <TmdbDisabledBanner reason={disabledReason} onSettings={() => navigate('/settings')} />
      )}

      <div className="flex min-h-0 flex-1 gap-3">
        <PreviewNamingWorkbenchSidebar
          selectedPreset={selectedPreset}
          currentNamingRule={currentNamingRule}
          currentStrategy={currentStrategy}
          folderPolicyConfig={folderPolicyConfig}
          setSelectedPreset={setSelectedPreset}
          setCurrentNamingRule={setCurrentNamingRule}
          setCurrentStrategy={setCurrentStrategy}
          triggerNamingApply={triggerNamingApply}
          onFolderPolicyChange={handleFolderPolicyChange}
        />

        <PreviewPlanTreePanel
          groups={groups}
          expandedGroups={expandedGroups}
          onGroupToggleExpand={toggleGroupExpanded}
          onGroupTmdbSearch={handleGroupTmdbSearch}
          onGroupEdit={openGroupEdit}
          onGroupSkip={skipGroup}
          onFileEdit={openFileEdit}
          onFileSkip={togglePreviewSkipped}
        />

        <TmdbInspectorSidebar
          selectedGroup={selectedGroup}
          tmdbMediaType={tmdbMediaType}
          tmdbQuery={tmdbQuery}
          tmdbLoading={tmdbLoading}
          tmdbError={tmdbError}
          tmdbDisabled={tmdbSearchStatus === 'disabled'}
          candidates={tmdbCandidates}
          selectedCandidate={selectedCandidate}
          onQueryChange={setTmdbQuery}
          onMediaTypeChange={setTmdbMediaType}
          onSearch={handleTmdbSearch}
          onCandidateSelect={setSelectedCandidate}
          onCandidateApply={handleApplyCandidate}
          onClearCandidates={clearCandidates}
        />
      </div>

      {editModal && (
        <EditNameModal
          mode={editModal.mode}
          currentName={editModal.currentName}
          previewPath={editModal.previewPath}
          onConfirm={confirmEdit}
          onCancel={cancelEdit}
        />
      )}

      <PreviewExecutionPanels
        uiState={uiState}
        confirmState={confirmState}
        progressState={progressState}
        resultState={resultState}
        onConfirm={confirmExecution}
        onCancel={cancelExecution}
        onReset={resetExecution}
      />
    </div>
  );
}
