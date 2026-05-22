# Hướng dẫn Demo Trao đổi Slot Môn học Ẩn danh

Hệ thống cho phép Sinh viên A và Sinh viên B trao đổi slot môn học mà không công khai địa chỉ ví của nhau trên bảng tin.

## 1. Chuẩn bị tài khoản
```bash
# Tạo tài khoản Admin, Sinh viên A, Sinh viên B
stellar keys generate admin --network testnet --fund
stellar keys generate student_a --network testnet --fund
stellar keys generate student_b --network testnet --fund
```

## 2. Build và Deploy Contract
```bash
# Di chuyển vào thư mục project
cd project

# Build contract
stellar contract build

# Deploy (Lưu CONTRACT_ID)
export CONTRACT_ID=$(stellar contract deploy \
  --wasm target/wasm32v1-none/release/course_swap.wasm \
  --source admin \
  --network testnet)

# Khởi tạo Admin
stellar contract invoke --id $CONTRACT_ID --source admin --network testnet -- \
  init --admin $(stellar keys address admin)
```

## 3. Quy trình Trao đổi Ẩn danh

### Bước A: Sinh viên A đăng nhu cầu
Sinh viên A có môn **CS101 (Rust)**, muốn đổi lấy môn **MATH202 (Toán)**.
```bash
stellar contract invoke --id $CONTRACT_ID --source student_a --network testnet -- \
  post_request \
  --user $(stellar keys address student_a) \
  --offered '{"course_id": "CS101", "course_name": "Lap trinh Rust", "class_id": "L01", "time_slot": "Mon 8:00"}' \
  --wanted '{"course_id": "MATH202", "course_name": "Toan Cao Cap", "class_id": "L05", "time_slot": "Fri 13:00"}'
# Kết quả trả về ID: 1
```

### Bước B: Sinh viên B xem bảng tin (Ẩn danh)
Sinh viên B có thể xem các tin đang có mà không biết ai là người đăng.
```bash
stellar contract invoke --id $CONTRACT_ID --source student_b --network testnet -- \
  get_available_swaps
# Kết quả chỉ hiển thị ID và thông tin môn học, không có 'owner'
```

### Bước C: Sinh viên B thực hiện đổi (Atomic Swap)
Sinh viên B cung cấp thông tin môn học mình đang có để đổi. Nếu B không có môn MATH202, giao dịch sẽ thất bại.
```bash
# Nếu đúng môn MATH202:
stellar contract invoke --id $CONTRACT_ID --source student_b --network testnet -- \
  execute_swap \
  --user $(stellar keys address student_b) \
  --request_id 1 \
  --user_course '{"course_id": "MATH202", "course_name": "Toan Cao Cap", "class_id": "L05", "time_slot": "Fri 13:00"}'
```

## 4. Kiểm tra sau giao dịch
```bash
# Bảng tin sẽ trống vì tin ID 1 đã hoàn thành
stellar contract invoke --id $CONTRACT_ID --source student_b --network testnet -- \
  get_available_swaps
```
