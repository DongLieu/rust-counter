// khai báo một module contract và làm cho module này có thể truy cập từ bên ngoài.
// Module này có thể chứa mã liên quan đến triển khai của hợp đồng thông minh,
// bao gồm cả hàm triển khai (init) và hàm xử lý các thông điệp từ người dùng (execute).
pub mod contract;

// Dòng này khai báo một module msg và làm cho module này có thể truy cập từ bên ngoài. 
// Module này có thể chứa các loại thông điệp mà hợp đồng có thể nhận và xử lý. Trong một hợp đồng
// thông minh, thông điệp thường được sử dụng để tương tác với hợp đồng thông qua giao diện người dùng hoặc thông qua các hợp đồng khác.
pub mod msg;

// Dòng này khai báo một module state và làm cho module này có thể truy cập từ bên ngoài. 
// Module này thường chứa mã liên quan đến trạng thái của hợp đồng, bao gồm cách lưu trữ và 
// truy cập các dữ liệu trong hợp đồng. Điều này bao gồm cả việc định nghĩa các cấu trúc dữ liệu để lưu trữ thông tin và các hàm để truy xuất và cập nhật trạng thái của hợp đồng.
pub mod state;