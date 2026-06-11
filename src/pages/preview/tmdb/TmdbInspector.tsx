import { CandidateList } from './CandidateList';
import { ScrapePreviewCard } from './ScrapePreviewCard';
import { TmdbEmptySelectionCard } from './TmdbEmptySelectionCard';
import { TmdbSearchCard } from './TmdbSearchCard';
import { TmdbErrorCard, TmdbLoadingCard } from './TmdbStatusCards';
import type { TmdbInspectorPanelProps } from './tmdbInspectorTypes';

export function TmdbInspector({
  selectedGroup,
  tmdbMediaType,
  tmdbQuery,
  tmdbLoading,
  tmdbError,
  tmdbDisabled,
  candidates,
  selectedCandidate,
  onQueryChange,
  onMediaTypeChange,
  onSearch,
  onCandidateSelect,
  onCandidateApply,
  onClearCandidates,
}: TmdbInspectorPanelProps) {
  if (!selectedGroup) {
    return <TmdbEmptySelectionCard />;
  }

  return (
    <>
      <TmdbSearchCard
        selectedGroup={selectedGroup}
        tmdbMediaType={tmdbMediaType}
        tmdbQuery={tmdbQuery}
        tmdbLoading={tmdbLoading}
        tmdbDisabled={tmdbDisabled}
        onQueryChange={onQueryChange}
        onMediaTypeChange={onMediaTypeChange}
        onSearch={onSearch}
      />

      {candidates.length > 0 && (
        <CandidateList
          candidates={candidates}
          selectedCandidate={selectedCandidate}
          onSelect={onCandidateSelect}
          onApply={onCandidateApply}
          onClear={onClearCandidates}
        />
      )}

      {tmdbLoading && <TmdbLoadingCard />}
      {tmdbError && <TmdbErrorCard message={tmdbError} />}
      {selectedCandidate && <ScrapePreviewCard selectedCandidate={selectedCandidate} selectedGroup={selectedGroup} />}
    </>
  );
}
