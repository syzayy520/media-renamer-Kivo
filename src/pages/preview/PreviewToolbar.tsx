import { Check, X, Filter } from 'lucide-react';
import { Button, Card, CardContent, Input } from '../../components/ui';

interface PreviewToolbarProps {
  filterText: string;
  onFilterTextChange: (text: string) => void;
  showOnlySelected: boolean;
  onToggleShowOnlySelected: () => void;
  selectedCount: number;
  filteredCount: number;
  onSelectAll: () => void;
}

export function PreviewToolbar({
  filterText,
  onFilterTextChange,
  showOnlySelected,
  onToggleShowOnlySelected,
  selectedCount,
  filteredCount,
  onSelectAll,
}: PreviewToolbarProps) {
  return (
    <Card variant="elevated" className="mb-6">
      <CardContent>
        <div className="flex flex-col md:flex-row gap-4">
          <div className="flex-1">
            <Input
              placeholder="搜索文件名..."
              value={filterText}
              onChange={(e) => onFilterTextChange(e.target.value)}
            />
          </div>
          <div className="flex gap-2">
            <Button
              variant={showOnlySelected ? 'primary' : 'secondary'}
              onClick={onToggleShowOnlySelected}
              icon={<Filter className="w-4 h-4" />}
            >
              已选择
            </Button>
            <Button
              variant="secondary"
              onClick={onSelectAll}
              icon={selectedCount === filteredCount ? 
                <X className="w-4 h-4" /> : 
                <Check className="w-4 h-4" />
              }
            >
              {selectedCount === filteredCount ? '取消全选' : '全选'}
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}