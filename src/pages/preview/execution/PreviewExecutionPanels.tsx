import type {
  ExecutionConfirmState,
  ExecutionProgressState,
  ExecutionResultState,
  ExecutionUIState,
} from '../../../types';
import { ExecutionConfirmPanel } from '../ExecutionConfirmPanel';
import { ExecutionProgressPanel } from '../ExecutionProgressPanel';
import { ExecutionResultPanel } from '../ExecutionResultPanel';

interface PreviewExecutionPanelsProps {
  uiState: ExecutionUIState;
  confirmState: ExecutionConfirmState;
  progressState: ExecutionProgressState;
  resultState: ExecutionResultState;
  onConfirm: () => Promise<void> | void;
  onCancel: () => void;
  onReset: () => void;
}

export function PreviewExecutionPanels({
  uiState,
  confirmState,
  progressState,
  resultState,
  onConfirm,
  onCancel,
  onReset,
}: PreviewExecutionPanelsProps) {
  return (
    <>
      {uiState === 'confirming' && (
        <ExecutionConfirmPanel
          state={confirmState}
          onConfirm={() => { void onConfirm(); }}
          onCancel={onCancel}
          isLoading={false}
        />
      )}
      {uiState === 'executing' && <ExecutionProgressPanel state={progressState} />}
      {uiState === 'completed' && (
        <ExecutionResultPanel
          state={resultState}
          onRollback={() => {}}
          onExport={() => {}}
          onReset={onReset}
        />
      )}
    </>
  );
}
