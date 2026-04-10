#!/usr/bin/env tsx
/**
 * Auto-discover local Claude Code skills and generate a summary report.
 * Usage: npx tsx scripts/scan-skills.ts [--json]
 */

import { loadAllSkills, getSkillsDir, listSkillNames } from '../src/providers/shared/skill-loader.js';

const jsonOutput = process.argv.includes('--json');

function main() {
  const skillsDir = getSkillsDir();
  const skillNames = listSkillNames();

  if (jsonOutput) {
    const skills = loadAllSkills();
    console.log(JSON.stringify(skills, null, 2));
    return;
  }

  console.log('=== Local Claude Code Skills Scan ===');
  console.log(`Skills directory: ${skillsDir}`);
  console.log(`Found: ${skillNames.length} skills\n`);

  const skills = loadAllSkills();

  for (const skill of skills) {
    console.log(`--- ${skill.name} ---`);
    console.log(`  Slug: ${skill.slug}`);
    console.log(`  Description: ${skill.description.slice(0, 120)}...`);
    console.log(`  Content length: ${skill.content.length} chars`);

    // Extract trigger keywords from description
    const triggers = extractTriggers(skill.description);
    if (triggers.length > 0) {
      console.log(`  Trigger keywords: ${triggers.join(', ')}`);
    }
    console.log('');
  }

  console.log('=== Summary ===');
  console.log(`Total skills: ${skills.length}`);
  console.log(`Total content: ${skills.reduce((sum, s) => sum + s.content.length, 0)} chars`);
}

function extractTriggers(description: string): string[] {
  // Extract quoted trigger phrases from skill descriptions
  const quoted = description.match(/"([^"]+)"|'([^']+)'/g) || [];
  return quoted
    .map(q => q.replace(/["']/g, ''))
    .filter(q => q.length > 1 && q.length < 30);
}

main();
