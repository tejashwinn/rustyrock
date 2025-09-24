pub mod options;
pub mod error;
pub mod key_value;
pub mod memtable;

#[cfg(test)]
mod tests {
	use crate::memtable::skiplist::SkipList;

	#[test]
	fn test_skiplist_basic() {
		let skiplist = SkipList::<i32, String>::new(4);
		assert_eq!(skiplist.estimated_size(), 0);
		assert!(skiplist.insert(1, "value1".to_string()));
		assert_eq!(skiplist.estimated_size(), 1);
	}
}