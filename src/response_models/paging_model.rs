// use serde::Serialize;

// #[derive(Serialize)]
// pub struct PagingModel<T> {
//     data: Vec<T>,
//     meta_data: PagingMetaData,
// }

// impl<T> PagingModel<T> {
//     pub fn new_paged(data: Vec<T>, page_size: u32, page_index: u32, total_count: u32) -> Self {
//         Self {
//             meta_data: PagingMetaData::new(page_size, page_index, total_count),
//             data: data,
//         }
//     }

//     pub fn new_without_paged(data: Vec<T>, page_size: u32, page_index: u32) -> Self {
//         Self {
//             meta_data: PagingMetaData::new(page_size, page_index, data.len() as u32),
//             data: Self::paging(data.into_iter(), page_size, page_index),
//         }
//     }

//     fn paging(iterator: impl Iterator<Item = T>, page_size: u32, page_index: u32) -> Vec<T> {
//         iterator
//             .skip((page_size * page_index) as usize)
//             .take(page_size as usize)
//             .collect()
//     }
// }

// #[derive(Serialize)]
// pub struct PagingMetaData {
//     page_size: u32,
//     page_index: u32,
//     has_next_page: bool,
//     has_previous_page: bool,
//     total_count: u32,
//     total_pages: u32,
// }

// impl PagingMetaData {
//     pub fn new(page_size: u32, page_index: u32, total_count: u32) -> Self {
//         let total_pages = Self::calculate_page_count(page_size, total_count);
//         return Self {
//             page_size: page_size,
//             page_index: page_index,
//             has_next_page: page_index < total_pages - 1,
//             has_previous_page: page_index > 0,
//             total_count: total_count,
//             total_pages: total_pages,
//         };
//     }

//     fn calculate_page_count(page_size: u32, total_count: u32) -> u32 {
//         let leap_page = match total_count % page_size {
//             0 => 0,
//             _ => 1,
//         };

//         total_count / page_size + leap_page
//     }
// }
