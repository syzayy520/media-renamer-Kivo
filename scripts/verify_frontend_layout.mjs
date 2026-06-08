#!/usr/bin/env node

/**
 * 程序化布局检查脚本
 * 验证前端页面在不同 viewport 下的布局正确性
 */

import { chromium } from 'playwright';

const VIEWPORTS = [
  { width: 900, height: 600 },
  { width: 1200, height: 800 },
  { width: 1600, height: 900 },
];

const PAGES = ['/', '/preview', '/audit'];

const BASE_URL = 'http://localhost:5173';

async function verifyLayout() {
  const browser = await chromium.launch({ headless: true });
  const results = [];

  try {
    for (const viewport of VIEWPORTS) {
      console.log(`\n=== 测试 viewport: ${viewport.width}x${viewport.height} ===`);

      const context = await browser.newContext({
        viewport: { width: viewport.width, height: viewport.height },
      });
      const page = await context.newPage();

      for (const pagePath of PAGES) {
        console.log(`\n--- 页面: ${pagePath} ---`);

        try {
          await page.goto(`${BASE_URL}${pagePath}`, {
            waitUntil: 'networkidle',
            timeout: 10000,
          });

          // 等待页面渲染
          await page.waitForTimeout(500);

          const checks = await runChecks(page, viewport);
          const allPassed = checks.every((c) => c.passed);

          if (!allPassed) {
            console.error(`❌ 页面 ${pagePath} 在 ${viewport.width}x${viewport.height} 下布局检查失败:`);
            checks
              .filter((c) => !c.passed)
              .forEach((c) => console.error(`  - ${c.name}: ${c.message}`));
            results.push({ viewport, page: pagePath, passed: false, checks });
          } else {
            console.log(`✅ 页面 ${pagePath} 在 ${viewport.width}x${viewport.height} 下布局检查通过`);
            results.push({ viewport, page: pagePath, passed: true, checks });
          }
        } catch (err) {
          console.error(`❌ 页面 ${pagePath} 加载失败:`, err.message);
          results.push({ viewport, page: pagePath, passed: false, error: err.message });
        }
      }

      await context.close();
    }
  } finally {
    await browser.close();
  }

  // 汇总结果
  const failed = results.filter((r) => !r.passed);
  console.log('\n=== 汇总 ===');
  console.log(`总检查数: ${results.length}`);
  console.log(`通过: ${results.length - failed.length}`);
  console.log(`失败: ${failed.length}`);

  if (failed.length > 0) {
    console.error('\n❌ 布局检查未通过');
    process.exit(1);
  } else {
    console.log('\n✅ 所有布局检查通过');
    process.exit(0);
  }
}

async function runChecks(page, viewport) {
  const checks = [];

  // 1. 页面标题 rect.width 必须 > 100（中文短标题如"重命名预览"5字约 108px）
  const title = await page.locator('h1, h2, h3').first();
  if (await title.isVisible().catch(() => false)) {
    const titleRect = await title.boundingBox();
    if (titleRect) {
      checks.push({
        name: '标题宽度',
        passed: titleRect.width > 100,
        message: `标题宽度 ${titleRect.width}px，要求 > 100px`,
      });

      // 2. 页面标题 rect.height 必须 < 80（防止竖排堆叠）
      checks.push({
        name: '标题高度',
        passed: titleRect.height < 80,
        message: `标题高度 ${titleRect.height}px，要求 < 80px`,
      });
    }
  }

  // 3. 主内容 main rect.width 必须接近 viewport 宽度，至少 > viewportWidth * 0.85
  const main = await page.locator('main').first();
  if (await main.isVisible()) {
    const mainRect = await main.boundingBox();
    const minWidth = viewport.width * 0.85;
    checks.push({
      name: '主内容宽度',
      passed: mainRect.width > minWidth,
      message: `主内容宽度 ${mainRect.width}px，要求 > ${minWidth}px (viewport ${viewport.width}px * 0.85)`,
    });
  } else {
    checks.push({
      name: '主内容可见性',
      passed: false,
      message: '主内容不可见',
    });
  }

  // 4. Scan 页面输入行 width 必须 > 600（在 900px viewport 下至少 > 500）
  // 只检查包含 input 的 flex 容器（即目录输入行），避免匹配按钮组
  const inputRow = await page.locator('.flex.gap-4:has(input)').first();
  if (await inputRow.isVisible().catch(() => false)) {
    const inputRowRect = await inputRow.boundingBox();
    if (inputRowRect) {
      const minInputWidth = viewport.width <= 900 ? 500 : 600;
      checks.push({
        name: '输入行宽度',
        passed: inputRowRect.width > minInputWidth,
        message: `输入行宽度 ${inputRowRect.width}px，要求 > ${minInputWidth}px`,
      });
    }
  }

  // 5. 浏览按钮 width 必须 > 56，height 必须 < 48
  const browseButton = await page.locator('button:has-text("浏览")').first();
  if (await browseButton.isVisible()) {
    const browseRect = await browseButton.boundingBox();
    checks.push({
      name: '浏览按钮宽度',
      passed: browseRect.width > 56,
      message: `浏览按钮宽度 ${browseRect.width}px，要求 > 56px`,
    });
    checks.push({
      name: '浏览按钮高度',
      passed: browseRect.height < 48,
      message: `浏览按钮高度 ${browseRect.height}px，要求 < 48px`,
    });
  }

  // 6. 开始扫描按钮 width 必须 > 88，height 必须 < 48
  const scanButton = await page.locator('button:has-text("开始扫描")').first();
  if (await scanButton.isVisible()) {
    const scanRect = await scanButton.boundingBox();
    checks.push({
      name: '开始扫描按钮宽度',
      passed: scanRect.width > 88,
      message: `开始扫描按钮宽度 ${scanRect.width}px，要求 > 88px`,
    });
    checks.push({
      name: '开始扫描按钮高度',
      passed: scanRect.height < 48,
      message: `开始扫描按钮高度 ${scanRect.height}px，要求 < 48px`,
    });
  }

  // 7. EmptyState 文案容器 width 必须 > 260
  // 只匹配 EmptyState 组件（包含 h3 的 flex-col 居中容器），避免匹配加载动画等
  const emptyState = await page.locator('.flex.flex-col.items-center.justify-center:has(h3)').first();
  if (await emptyState.isVisible().catch(() => false)) {
    const emptyStateRect = await emptyState.boundingBox();
    if (emptyStateRect) {
      checks.push({
        name: 'EmptyState 宽度',
        passed: emptyStateRect.width > 260,
        message: `EmptyState 宽度 ${emptyStateRect.width}px，要求 > 260px`,
      });

      // 8. EmptyState 文案 height 不得异常大，例如不能超过 200
      checks.push({
        name: 'EmptyState 高度',
        passed: emptyStateRect.height < 200,
        message: `EmptyState 高度 ${emptyStateRect.height}px，要求 < 200px`,
      });
    }
  }

  // 9. nav 不得把内容挤成竖排
  const nav = await page.locator('nav').first();
  if (await nav.isVisible()) {
    const navRect = await nav.boundingBox();
    checks.push({
      name: '导航栏高度',
      passed: navRect.height < 100,
      message: `导航栏高度 ${navRect.height}px，要求 < 100px（防止竖排）`,
    });
  }

  // 10. document.body.scrollWidth 不得明显大于 window.innerWidth + 20
  const scrollWidth = await page.evaluate(() => document.body.scrollWidth);
  const innerWidth = await page.evaluate(() => window.innerWidth);
  checks.push({
    name: '页面水平溢出',
    passed: scrollWidth <= innerWidth + 20,
    message: `scrollWidth ${scrollWidth}px, innerWidth ${innerWidth}px, 差值 ${scrollWidth - innerWidth}px，要求 <= 20px`,
  });

  return checks;
}

verifyLayout().catch((err) => {
  console.error('布局检查脚本执行失败:', err);
  process.exit(1);
});
