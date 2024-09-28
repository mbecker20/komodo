import { cn } from "@lib/utils";
import {
  Column,
  ColumnDef,
  SortingState,
  flexRender,
  getCoreRowModel,
  getSortedRowModel,
  useReactTable,
} from "@tanstack/react-table";

import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@ui/table";
import { ArrowDown, ArrowUp, Minus } from "lucide-react";
import { ReactNode, useEffect, useState } from "react";

interface DataTableProps<TData, TValue> {
  /** Unique key given to table so sorting can be remembered on local storage */
  tableKey: string;
  columns: (ColumnDef<TData, TValue> | false | undefined)[];
  data: TData[];
  onRowClick?: (row: TData) => void;
  noResults?: ReactNode;
  defaultSort?: SortingState;
  sortDescFirst?: boolean;
}

export function DataTable<TData, TValue>({
  tableKey,
  columns,
  data,
  onRowClick,
  noResults,
  sortDescFirst = false,
  defaultSort = [],
}: DataTableProps<TData, TValue>) {
  const [sorting, setSorting] = useState<SortingState>(defaultSort);

  const table = useReactTable({
    data,
    columns: columns.filter((c) => c) as any,
    getCoreRowModel: getCoreRowModel(),
    onSortingChange: setSorting,
    getSortedRowModel: getSortedRowModel(),
    state: {
      sorting,
    },
    sortDescFirst,
  });

  useEffect(() => {
    const stored = localStorage.getItem("data-table-" + tableKey);
    const sorting = stored ? (JSON.parse(stored) as SortingState) : null;
    if (sorting) setSorting(sorting);
  }, [tableKey]);

  useEffect(() => {
    if (sorting.length) {
      localStorage.setItem("data-table-" + tableKey, JSON.stringify(sorting));
    }
  }, [tableKey, sorting]);

  return (
    <div className="rounded-xl border bg-card text-card-foreground shadow p-2">
      <Table className="xl:table-fixed border-separate border-spacing-0">
        <TableHeader className="sticky top-0">
          {table.getHeaderGroups().map((headerGroup) => (
            <TableRow key={headerGroup.id}>
              {headerGroup.headers.map((header) => {
                return (
                  <TableHead
                    key={header.id}
                    colSpan={header.colSpan}
                    className={cn(
                      "relative whitespace-nowrap bg-background border-b border-r last:border-r-0"
                      // `w-[${header.column.getSize()}px]`
                    )}
                  >
                    {header.isPlaceholder
                      ? null
                      : flexRender(
                          header.column.columnDef.header,
                          header.getContext()
                        )}
                  </TableHead>
                );
              })}
            </TableRow>
          ))}
        </TableHeader>
        <TableBody>
          {table.getRowModel().rows?.length ? (
            table.getRowModel().rows.map((row) => (
              <TableRow
                key={row.id}
                data-state={row.getIsSelected() && "selected"}
                onClick={() => onRowClick && onRowClick(row.original)}
                className={cn(
                  "even:bg-accent/25",
                  onRowClick && "cursor-pointer"
                )}
              >
                {row.getVisibleCells().map((cell) => {
                  const size = cell.column.getSize();
                  return (
                    <TableCell
                      key={cell.id}
                      // className="p-4 border-x first:border-r first:border-l-0 last:border-l last:border-r-0"
                      className={cn(
                        "p-4 overflow-hidden overflow-ellipsis",
                        size && `w-[${size}px]`
                      )}
                    >
                      {flexRender(
                        cell.column.columnDef.cell,
                        cell.getContext()
                      )}
                    </TableCell>
                  );
                })}
              </TableRow>
            ))
          ) : (
            <TableRow>
              <TableCell colSpan={columns.length} className="p-4 text-center">
                {noResults ?? "No results."}
              </TableCell>
            </TableRow>
          )}
        </TableBody>
      </Table>
    </div>
  );
}

export const SortableHeader = <T, V>({
  column,
  title,
  sortDescFirst,
}: {
  column: Column<T, V>;
  title: string;
  sortDescFirst?: boolean;
}) => (
  <div
    className="flex items-center justify-between"
    onClick={() => column.toggleSorting()}
  >
    {title}
    {column.getIsSorted() === "asc" ? (
      sortDescFirst ? (
        <ArrowUp className="w-4" />
      ) : (
        <ArrowDown className="w-4" />
      )
    ) : column.getIsSorted() === "desc" ? (
      sortDescFirst ? (
        <ArrowDown className="w-4" />
      ) : (
        <ArrowUp className="w-4" />
      )
    ) : (
      <Minus className="w-4" />
    )}
  </div>
);
