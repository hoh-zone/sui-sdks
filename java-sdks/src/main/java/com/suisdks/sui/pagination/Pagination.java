package com.suisdks.sui.pagination;

import java.util.List;
import java.util.function.Function;

public class Pagination<T> {
    public interface QueryFunction<T> {
        List<T> query(String cursor, long limit);
    }

    public static <T> PaginatedIterator<T> all(QueryFunction<T> queryFunction, long limit) {
        return new PaginatedIterator<>(queryFunction, limit);
    }

    public static <T> PaginatedIterator<T> all(QueryFunction<T> queryFunction) {
        return new PaginatedIterator<>(queryFunction, 100);
    }

    public static <T, R> PaginatedIterator<R> map(PaginatedIterator<T> source, Function<T, R> mapper) {
        return new MappedIterator<>(source, mapper);
    }

    public static class PaginatedIterator<T> implements java.util.Iterator<T> {
        private final QueryFunction<T> queryFunction;
        private final long limit;
        private List<T> items;
        private int currentIndex;
        private String nextCursor;
        private boolean hasNextPage;
        private boolean exhausted;

        public PaginatedIterator(QueryFunction<T> queryFunction, long limit) {
            this.queryFunction = queryFunction;
            this.limit = limit;
            this.items = List.of();
            this.currentIndex = 0;
            this.hasNextPage = true;
            this.exhausted = false;
            fetchNextPage();
        }

        private void fetchNextPage() {
            if (!hasNextPage || exhausted) {
                exhausted = true;
                return;
            }

            List<T> result = queryFunction.query(nextCursor, limit);
            items = result;
            currentIndex = 0;
            hasNextPage = false;

            if (items.isEmpty()) {
                exhausted = true;
                nextCursor = null;
            }
        }

        @Override
        public boolean hasNext() {
            return !exhausted && currentIndex < items.size();
        }

        @Override
        public T next() {
            if (!hasNext()) {
                throw new java.util.NoSuchElementException();
            }
            T item = items.get(currentIndex++);

            if (currentIndex >= items.size() && hasNextPage) {
                fetchNextPage();
            } else if (currentIndex >= items.size()) {
                exhausted = true;
            }

            return item;
        }
    }

    private static class MappedIterator<T, R> implements java.util.Iterator<R> {
        private final PaginatedIterator<T> source;
        private final Function<T, R> mapper;

        public MappedIterator(PaginatedIterator<T> source, Function<T, R> mapper) {
            this.source = source;
            this.mapper = mapper;
        }

        @Override
        public boolean hasNext() {
            return source.hasNext();
        }

        @Override
        public R next() {
            return mapper.apply(source.next());
        }
    }
}