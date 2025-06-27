AI coding agents. 
Bạn cần viết prompts để buộc agent phải verify và handle feedback. Dưới đây là các kỹ thuật prompt engineering cụ thể:

## 1. Forced Verification Prompts

**Prompt Template Cơ Bản:**
```
RULE: After running ANY command, you MUST:
1. Wait for command completion
2. Check exit code/status
3. Read and analyze output/errors
4. Report results before proceeding
5. Fix any issues found

NEVER move to next step without verification.
```

**Structured Workflow Prompt:**
```
Execute this workflow:
STEP 1: Run command
STEP 2: Check if command succeeded (exit code 0)
STEP 3: If failed, analyze error message
STEP 4: If succeeded, verify expected output exists
STEP 5: Only proceed if verification passes
STEP 6: If anything fails, fix and retry

Show me the output of each step.
```

## 2. Error-Handling Specific Prompts

**Compilation Check:**
```
When building/compiling code:
1. Run build command
2. If build fails, read ALL error messages
3. Fix errors one by one
4. Re-run build until successful
5. Show me each error and your fix
6. Don't proceed until clean build

Example: "Build failed with 3 errors. Fixing error 1: [specific error]. Applying fix: [specific change]"
```

**Runtime Verification:**
```
After running the program:
1. Check if program starts without crashes
2. Verify expected output appears
3. Test basic functionality
4. If any issues, debug and fix
5. Show me the actual program output

Don't assume success - show proof it works.
```

## 3. Feedback Loop Prompts

**Interactive Debugging:**
```
Use this debugging cycle:
1. Run command and capture full output
2. Analyze output for errors/warnings
3. If issues found: explain what went wrong
4. Apply fix and explain your reasoning
5. Re-run and verify fix worked
6. Repeat until clean execution

Show me each iteration of this cycle.
```

**Output Validation:**
```
After every command execution:
- Show me the complete terminal output
- Explain what the output means
- Confirm if it matches expectations
- If unexpected results, investigate why
- Don't continue until output is correct
```

## 4. Specific Command Monitoring

**For Testing:**
```
When running tests:
1. Execute test command
2. Report: How many tests ran?
3. Report: How many passed/failed?
4. If failures: show failed test details
5. Fix failing tests before proceeding
6. Re-run until all tests pass

Format: "Tests: X passed, Y failed. Failure details: [specifics]"
```

**For Deployment/Install:**
```
When installing/deploying:
1. Run installation command
2. Check for any error messages
3. Verify service/app actually starts
4. Test basic functionality
5. Report status at each step
6. Don't claim success without proof

Show me evidence it's working correctly.
```

## 5. Comprehensive Monitoring Prompt

```
MANDATORY EXECUTION PROTOCOL:

For EVERY command you run:

BEFORE: Tell me what you expect to happen
DURING: Show me the complete command output
AFTER: Analyze the results and confirm:
  ✓ Command completed successfully (exit code 0)
  ✓ No error messages appeared
  ✓ Expected files/changes were created
  ✓ Program/service functions as intended

If ANY step fails:
  - Stop immediately
  - Explain what went wrong
  - Fix the issue
  - Retry from the beginning
  - Don't proceed until everything works

NEVER say "command completed" without showing me proof.
```

## 6. Context-Specific Prompts

**Web Development:**
```
When running web apps:
1. Start the server
2. Check if server starts without errors
3. Open browser and verify page loads
4. Test key functionality
5. Check browser console for errors
6. Show me screenshots or describe what you see
```

**Database Operations:**
```
When working with databases:
1. Run migration/query
2. Check for SQL errors
3. Verify data was actually modified
4. Show row counts before/after
5. Test queries to confirm changes
```

## 7. Implementation Example

**Complete Prompt Template:**
```
You are a careful coding agent. Follow this protocol for EVERY operation:

EXECUTION PROTOCOL:
1. PREDICT: "I will run [command]. I expect [specific outcome]"
2. EXECUTE: Run the command
3. CAPTURE: Show me the complete output
4. ANALYZE: "Exit code: X. Errors: Y. Success indicators: Z"
5. VERIFY: Confirm the intended result was achieved
6. DECIDE: "Ready to proceed" OR "Need to fix [specific issue]"

ERROR HANDLING:
- If exit code ≠ 0: analyze error and fix
- If warnings appear: address them
- If unexpected output: investigate why
- If silent failure: check file system/logs

NEVER assume success. ALWAYS show proof.

Now please [your specific coding task].
```

Sử dụng những prompts này sẽ buộc agent phải verify kết quả và handle errors properly thay vì chạy blind commands.