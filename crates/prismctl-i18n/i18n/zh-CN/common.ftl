# 通用文本
app-name = Prismctl
app-subtitle = 交互式设置向导

# 通用操作
action-continue = 按回车继续...
action-confirm = 确认写入？输入 "yes" 继续（其它任意键取消）:
action-confirm-apply = 确认写入？
action-confirm-create = 确认创建？
action-cancel = 已取消

# 通用错误
error-empty-input = 输入不能为空
error-cancelled = 已取消（Ctrl+C）
error-cancelled-esc = 已取消（ESC）
error-invalid-choice = 无效选择: { $choice }
error-io = 输出失败: { $error }
error-read-input = 读取输入失败: { $error }

# 交互/输入
error-interactive-input = 交互式输入失败: { $error }
info-env-default-detected = （已检测到环境变量默认值；留空=使用默认值；输入 "-"=跳过不写入）

# inquire（交互组件提示）
inquire-help-select = 使用 Up/Down 移动，Enter 确认
inquire-help-multi-select = 使用 Up/Down 移动，Space 切换，Enter 确认
inquire-help-confirm = 使用 Y/N，Enter 确认

# 校验
error-url-empty = URL 不能为空
error-url-invalid = URL 格式不合法（需要以 http:// 或 https:// 开头）：{ $url }
error-api-key-empty = API Key 不能为空
error-api-key-whitespace = API Key 不能包含空白字符
error-api-key-too-short = API Key 长度过短（可能输入有误）
