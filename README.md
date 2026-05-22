# CourseSwap - Anonymous Course Slot Exchange

## Problem
Sinh viên gặp khó khăn và rủi ro khi tìm người đổi lịch học thủ công, dễ bị lừa đảo hoặc mất slot vào tay các "bot" săn môn công cộng.

## Solution
Một nền tảng trao đổi slot môn học phi tập trung sử dụng Smart Contract để đảm bảo tính **nguyên tử (Atomic)**: Giao dịch đổi môn chỉ thành công khi cả hai bên cung cấp đúng môn học đối phương cần.

## Why Stellar
Stellar Soroban cung cấp phí giao dịch cực thấp và tính năng **Atomic Swap** tích hợp, cho phép hoán đổi quyền sở hữu môn học một cách an toàn, tức thì và ẩn danh.

## Target User
Sinh viên đại học có nhu cầu thay đổi lịch học, đổi lớp hoặc đổi môn học trong các kỳ đăng ký tín chỉ.

## Live Demo
- **Network**: Stellar Testnet
- **Contract ID**: `CAT2HAZFNSCNCG4OGII7GX4SELLGIW2NMDCLPCNCNKCMZWOJR76J3SSZ`
- **Transaction**: [View on Stellar Expert](https://stellar.expert/explorer/testnet/tx/420e32cfa14bfd383364f54f1f9872d1f08ffe37aa638b160790be85baf6e3f3)

## How to Run
1. **Clone**: `git clone https://github.com/yourname/course-swap.git`
2. **Build**: 
   ```bash
   cd project/contracts/course-swap
   stellar contract build
   ```
3. **Test**: 
   ```bash
   cargo test -p course-swap
   ```
4. **Deploy**:
   ```bash
   stellar contract deploy \
     --wasm project/target/wasm32v1-none/release/course_swap.wasm \
     --source admin \
     --network testnet
   ```
5. **Frontend**: 
   Mở file `project/index.html` bằng trình duyệt (Yêu cầu cài đặt Freighter Wallet).

## Tech Stack
- **Smart Contract**: Rust / Soroban SDK v25
- **Frontend**: HTML5 / JavaScript / @stellar/stellar-sdk
- **Wallet**: Freighter
- **Network**: Stellar Testnet

## Team
- [Tên của bạn] | [@telegram] | [email] | [Tên Trường - Năm thứ X]
