#!/usr/bin/env python3
"""
Script to fix SeaORM entity files by removing invalid #[sea_orm::model] attributes
and relationship fields from Model structs.
"""

import re
import os
from pathlib import Path

def fix_entity_file(file_path):
    """Fix a single entity file."""
    with open(file_path, 'r') as f:
        content = f.read()
    
    original_content = content
    
    # Step 1: Remove #[sea_orm::model] line
    content = re.sub(r'#\[sea_orm::model\]\n', '', content)
    
    # Step 2: Find and process the Model struct
    # Match from "pub struct Model {" to the closing "}"
    pattern = r'(pub struct Model\s*\{)(.*?)(\n\})'
    
    def process_struct(match):
        prefix = match.group(1)
        body = match.group(2)
        suffix = match.group(3)
        
        lines = body.split('\n')
        new_lines = []
        i = 0
        
        while i < len(lines):
            line = lines[i]
            
            # Check if this line starts an attribute
            if line.strip().startswith('#[sea_orm('):
                # Collect the full attribute (may span multiple lines)
                attr_lines = [line]
                
                # Keep collecting until we find )]
                while not attr_lines[-1].rstrip().endswith(')]'):
                    i += 1
                    if i >= len(lines):
                        break
                    attr_lines.append(lines[i])
                
                i += 1  # Move past the last attribute line
                
                # Now we need to find the field definition
                # It might be on the next line, or there might be blank lines
                field_lines = []
                
                # Collect lines until we find one with a comma or are at struct end
                while i < len(lines):
                    current = lines[i]
                    
                    # Skip blank lines
                    if not current.strip():
                        field_lines.append(current)
                        i += 1
                        continue
                    
                    # Add this line to field_lines
                    field_lines.append(current)
                    i += 1
                    
                    # Check if we've completed a field (ends with comma or is last field)
                    if current.rstrip().endswith(','):
                        break
                    
                    # Check if next line is an attribute or end of struct
                    if i < len(lines):
                        next_line = lines[i].strip()
                        if next_line.startswith('#[') or next_line == '':
                            # Continue to see if there's more to this field
                            continue
                        # If next line doesn't start with whitespace or special char, field is complete
                        if not lines[i].startswith((' ', '\t')) or next_line.startswith('pub '):
                            break
                
                # Join all field lines to analyze
                full_field = '\n'.join(field_lines)
                full_attr = '\n'.join(attr_lines)
                
                # Check if this is a relationship attribute OR if the field is a relationship type
                is_relationship_attr = bool(re.search(r'#\[sea_orm\([^)]*(?:has_many|has_one|self_ref)', full_attr, re.DOTALL))
                is_relationship_field = bool(re.search(r':\s*Has(?:Many|One)<', full_field))
                has_belongs_to = bool(re.search(r'#\[sea_orm\([^)]*belongs_to', full_attr, re.DOTALL))
                
                if is_relationship_attr or is_relationship_field:
                    # Skip this attribute and field
                    continue
                elif has_belongs_to:
                    # This is a belongs_to attribute on a regular field
                    # We need to extract primary_key and auto_increment if present, and keep the field
                    # Parse the attribute to extract important parts
                    has_primary_key = bool(re.search(r'\bprimary_key\b', full_attr))
                    has_auto_increment = bool(re.search(r'\bauto_increment\s*=\s*false\b', full_attr))
                    
                    if has_primary_key or has_auto_increment:
                        # Reconstruct a minimal attribute with only the important parts
                        attr_parts = []
                        if has_primary_key:
                            attr_parts.append('primary_key')
                        if has_auto_increment:
                            attr_parts.append('auto_increment = false')
                        
                        if attr_parts:
                            # Get the indentation from the first attribute line
                            indent_match = re.match(r'^(\s*)', attr_lines[0])
                            indent = indent_match.group(1) if indent_match else '    '
                            new_attr = f'{indent}#[sea_orm({", ".join(attr_parts)})]'
                            new_lines.append(new_attr)
                    
                    # Keep the field
                    new_lines.extend(field_lines)
                    continue
                else:
                    # Keep this attribute and field
                    new_lines.extend(attr_lines)
                    new_lines.extend(field_lines)
                    continue
            
            # Check if this is a standalone field line (no preceding attribute)
            # Could be start of a multi-line field
            elif line.strip().startswith('pub '):
                field_lines = [line]
                i += 1
                
                # Check if this line has a complete field definition
                if ':' in line and ',' in line:
                    # Single line field
                    if re.search(r':\s*Has(?:Many|One)<', line):
                        # Skip relationship field
                        continue
                    else:
                        # Keep regular field
                        new_lines.append(line)
                        continue
                
                # Multi-line field - collect until we find the end
                while i < len(lines):
                    current = lines[i]
                    field_lines.append(current)
                    i += 1
                    
                    if current.rstrip().endswith(','):
                        break
                    
                    # Check if we're at the end
                    if i < len(lines):
                        next_line = lines[i].strip()
                        if next_line.startswith('#[') or next_line.startswith('pub '):
                            break
                
                # Check if this is a relationship field
                full_field = '\n'.join(field_lines)
                if re.search(r':\s*Has(?:Many|One)<', full_field):
                    # Skip it
                    continue
                else:
                    # Keep it
                    new_lines.extend(field_lines)
                    continue
            
            # Check for orphaned HasMany/HasOne lines (without pub prefix)
            elif re.search(r'^\s*Has(?:Many|One)<', line):
                # Skip this orphaned relationship line
                i += 1
                continue
            
            else:
                # Keep other lines (comments, blank lines, etc.)
                new_lines.append(line)
                i += 1
        
        return prefix + '\n'.join(new_lines) + suffix
    
    content = re.sub(pattern, process_struct, content, flags=re.DOTALL)
    
    # Step 3: Add empty Relation enum if not present
    # Check if file already has a Relation enum
    if not re.search(r'pub enum Relation', content):
        # Find the ActiveModelBehavior implementation and add Relation enum before it
        relation_code = '''
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

'''
        # Insert before ActiveModelBehavior
        content = re.sub(
            r'(impl ActiveModelBehavior for ActiveModel)',
            relation_code + r'\1',
            content
        )
    
    # Only write if content changed
    if content != original_content:
        with open(file_path, 'w') as f:
            f.write(content)
        return True
    return False

def main():
    """Fix all entity files in the entity/src directory."""
    entity_dir = Path(__file__).parent / 'entity' / 'src'
    
    if not entity_dir.exists():
        print(f"Error: {entity_dir} does not exist")
        return
    
    # Get all .rs files except lib.rs, prelude.rs, and sea_orm_active_enums.rs
    exclude_files = {'lib.rs', 'prelude.rs', 'sea_orm_active_enums.rs'}
    entity_files = [
        f for f in entity_dir.glob('*.rs')
        if f.name not in exclude_files
    ]
    
    print(f"Found {len(entity_files)} entity files to process")
    
    fixed_count = 0
    for file_path in sorted(entity_files):
        if fix_entity_file(file_path):
            fixed_count += 1
            print(f"Fixed: {file_path.name}")
    
    print(f"\nFixed {fixed_count} out of {len(entity_files)} files")

if __name__ == '__main__':
    main()