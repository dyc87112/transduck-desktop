#!/bin/bash

# 设置图标目录
ICONS_DIR="src-tauri/icons"

# 创建临时目录
TEMP_DIR="temp_icons"
mkdir -p $TEMP_DIR

# 处理每个图标文件
for icon in $ICONS_DIR/*.png $ICONS_DIR/*.ico $ICONS_DIR/*.icns; do
  if [ -f "$icon" ]; then
    filename=$(basename "$icon")
    echo "处理图标: $filename"
    
    # 获取图像尺寸
    size=$(identify -format "%wx%h" "$icon" 2>/dev/null)
    if [ $? -ne 0 ]; then
      echo "跳过非图像文件: $filename"
      continue
    fi
    
    # 创建白色背景
    convert -size $size xc:white "$TEMP_DIR/bg_$filename"
    
    # 叠加原始图标到白色背景
    convert "$TEMP_DIR/bg_$filename" "$icon" -composite "$TEMP_DIR/$filename"
    
    # 替换原始图标
    mv "$TEMP_DIR/$filename" "$icon"
  fi
done

# 清理临时目录
rm -rf $TEMP_DIR

echo "所有图标处理完成！"