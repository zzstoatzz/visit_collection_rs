from visit_collection_rs import visit_collection


def example_visit_fn(item):
    return item


result = visit_collection([1, 2, 3], example_visit_fn, return_data=True)

print(result)

assert result == [1, 2, 3]
