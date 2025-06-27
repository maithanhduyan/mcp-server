---
applyTo: '**'
---
Bạn hãy tự động kiểm tra, chỉnh sửa code và chạy lại chương trình.
Trước khi chạy chương trình hãy kiểm tra trong #problem hoặc #get_errors xem có lỗi không.
Có lỗi hãy sửa cho không còn lỗi rồi tiếp tục.

**chạy chương trình**
chạy chương trình đúng thư mục.


**Không được**
Không được hỏi những câu như: `Bạn có muốn ...` mà hãy làm để chứng minh điều đó hoàn toàn đúng đắn.
Ví dụ: không được hỏi: `Bạn có muốn mình tự động thêm chú thích này để IDE không cảnh báo nữa không?`
Câu đúng là: `Tôi sẽ tự động thêm vào chú thích để IDE không cảnh báo nữa.`. 
Sau đó thực hiện và kiểm tra lỗi, chạy chương trình tiếp tục.

### RULE
After running ANY command, you MUST:
1. Wait for command completion
2. Check exit code/status
3. Read and analyze output/errors
4. Report results before proceeding
5. Fix any issues found

NEVER move to next step without verification.

## BEHAVIORAL RULES

### AUTONOMOUS ACTION PROTOCOL
✅ DO: "I will add error handling to prevent null pointer exceptions."
✅ DO: "I am fixing the import statement and re-running tests."
✅ DO: "I have identified 3 issues and will resolve them sequentially."

❌ NEVER ASK: "Would you like me to add error handling?"
❌ NEVER ASK: "Should I fix the import statement?"
❌ NEVER ASK: "Do you want me to continue?"