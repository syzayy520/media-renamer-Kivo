import { useState, useMemo } from 'react';
import { ArrowLeft, ArrowRight } from 'lucide-react';
import { Button, Card } from '../../components/ui';
import { usePipelineStore } from '../../state/pipelineStore';
import { useUiFeedbackStore } from '../../state/uiFeedbackStore';
import { PreviewToolbar } from './PreviewToolbar';
import { PreviewTable } from './PreviewTable';
import { PreviewActionBar } from './PreviewActionBar';
import { PreviewEmptyState } from './PreviewEmptyState';

export function PreviewPage() {
  const { pipelineResult } = usePipelineStore();
  const { isLoading } = useUiFeedbackStore();
  const [selectedItems, setSelectedItems] = useState<Set<string>>(new Set());
  const [filterText, setFilterText] = useState('');
  const [showOnlySelected, setShowOnlySelected] = useState(false);

  const previews = useMemo(() => pipelineResult?.previews ?? [], [pipelineResult]);

  const filteredItems = useMemo(() => {
    return previews.filter((item) => {
      const matchesFilter = filterText === '' || 
        item.original_name.toLowerCase().includes(filterText.toLowerCase()) ||
        item.proposed_name.toLowerCase().includes(filterText.toLowerCase());
      const matchesSelection = !showOnlySelected || selectedItems.has(item.id);
      return matchesFilter && matchesSelection;
    });
  }, [previews, filterText, showOnlySelected, selectedItems]);

  if (!pipelineResult) {
    return <PreviewEmptyState />;
  }

  const handleSelectItem = (id: string) => {
    const newSelected = new Set(selectedItems);
    if (newSelected.has(id)) {
      newSelected.delete(id);
    } else {
      newSelected.add(id);
    }
    setSelectedItems(newSelected);
  };

  const handleSelectAll = () => {
    if (selectedItems.size === filteredItems.length) {
      setSelectedItems(new Set());
    } else {
      setSelectedItems(new Set(filteredItems.map(item => item.id)));
    }
  };

  const selectedCount = selectedItems.size;

  return (
    <div className="w-full px-6 py-8">
      <div className="w-full">
        <div className="flex items-center justify-between mb-8 gap-4">
          <div className="min-w-0 flex-1">
            <h1 className="text-3xl font-bold text-text-primary whitespace-nowrap">
              重命名预览
            </h1>
            <p className="text-text-secondary mt-2">
              共 {previews.length} 个文件，已选择 {selectedCount} 个
            </p>
          </div>
          <div className="flex gap-4 shrink-0">
            <Button variant="secondary" icon={<ArrowLeft className="w-4 h-4" />}>
              返回
            </Button>
            <Button icon={<ArrowRight className="w-4 h-4" />}>
              执行重命名
            </Button>
          </div>
        </div>

        <PreviewToolbar
          filterText={filterText}
          onFilterTextChange={setFilterText}
          showOnlySelected={showOnlySelected}
          onToggleShowOnlySelected={() => setShowOnlySelected(!showOnlySelected)}
          selectedCount={selectedCount}
          filteredCount={filteredItems.length}
          onSelectAll={handleSelectAll}
        />

        <Card variant="elevated">
          <PreviewTable
            items={filteredItems}
            selectedItems={selectedItems}
            onSelectItem={handleSelectItem}
            onSelectAll={handleSelectAll}
            filteredCount={filteredItems.length}
          />
        </Card>

        <PreviewActionBar
          selectedCount={selectedCount}
          onClearSelection={() => setSelectedItems(new Set())}
        />

        {isLoading && (
          <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
            <div className="bg-bg-card p-6 rounded-lg">
              <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-accent mx-auto mb-4"></div>
              <p className="text-text-primary">正在处理...</p>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}