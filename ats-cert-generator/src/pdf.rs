use crate::CertificateData;
use js_sys::Math;

/// Simple PDF generator for WASM environment
/// Creates a minimal PDF with certificate data
pub struct PdfGenerator {
    objects: Vec<String>,
    current_obj_id: usize,
    xref_positions: Vec<usize>,
    content: Vec<u8>,
}

impl PdfGenerator {
    pub fn new() -> Self {
        PdfGenerator {
            objects: Vec::new(),
            current_obj_id: 1,
            xref_positions: Vec::new(),
            content: Vec::new(),
        }
    }

    fn add_object(&mut self, content: String) -> usize {
        let obj_id = self.current_obj_id;
        self.current_obj_id += 1;
        
        let obj = format!("{} 0 obj\n{}\nendobj\n", obj_id, content);
        self.xref_positions.push(self.content.len());
        self.content.extend_from_slice(obj.as_bytes());
        self.objects.push(obj);
        
        obj_id
    }

    pub fn generate_certificate_pdf(data: &CertificateData) -> Result<Vec<u8>, String> {
        let mut gen = PdfGenerator::new();
        
        // PDF header
        gen.content.extend_from_slice(b"%PDF-1.4\n");
        gen.content.extend_from_slice(&[0x25, 0xE2, 0xE3, 0xCF, 0xD3, 0x0A]); // Binary marker
        
        // Reserve object IDs for proper references
        let catalog_ref = 1;
        let pages_ref = 2;
        let page_ref = 3;
        let font_ref = 4;
        let font_bold_ref = 5;
        let stream_ref = 6;
        
        // 1. Catalog
        let catalog = format!("<<\n/Type /Catalog\n/Pages {} 0 R\n>>", pages_ref);
        gen.add_object(catalog);
        
        // 2. Pages
        let pages = format!(
            "<<\n/Type /Pages\n/Kids [{} 0 R]\n/Count 1\n>>",
            page_ref
        );
        gen.add_object(pages);
        
        // 3. Page
        let page = format!(
            "<<\n/Type /Page\n/Parent {} 0 R\n/MediaBox [0 0 595 842]\n/Resources <<\n/Font <<\n/F1 {} 0 R\n/F2 {} 0 R\n>>\n>>\n/Contents {} 0 R\n>>",
            pages_ref,
            font_ref,
            font_bold_ref,
            stream_ref
        );
        gen.add_object(page);
        
        // 4. Font (regular)
        let font = "<<\n/Type /Font\n/Subtype /Type1\n/BaseFont /Times-Roman\n>>";
        gen.add_object(font.to_string());
        
        // 5. Font (bold)
        let font_bold = "<<\n/Type /Font\n/Subtype /Type1\n/BaseFont /Times-Bold\n>>";
        gen.add_object(font_bold.to_string());
        
        // 6. Content stream
        let content_stream = gen.create_content_stream(data, font_ref, font_bold_ref);
        let content_stream_bytes = content_stream.as_bytes();
        let stream_obj = format!(
            "<<\n/Length {}\n>>\nstream\n{}\nendstream",
            content_stream_bytes.len(),
            content_stream
        );
        gen.add_object(stream_obj);
        
        // Cross-reference table
        let xref_start = gen.content.len();
        gen.content.extend_from_slice(b"xref\n");
        gen.content.extend_from_slice(format!("0 {}\n", gen.current_obj_id).as_bytes());
        gen.content.extend_from_slice(b"0000000000 65535 f \n");
        
        for pos in &gen.xref_positions {
            gen.content.extend_from_slice(format!("{:010} 00000 n \n", pos).as_bytes());
        }
        
        // Trailer
        gen.content.extend_from_slice(b"trailer\n");
        gen.content.extend_from_slice(
            format!(
                "<<\n/Size {}\n/Root {} 0 R\n>>\n",
                gen.current_obj_id,
                catalog_ref  // This is 1
            )
            .as_bytes(),
        );
        
        // Footer
        gen.content.extend_from_slice(b"startxref\n");
        gen.content.extend_from_slice(format!("{}\n", xref_start).as_bytes());
        gen.content.extend_from_slice(b"%%EOF\n");
        
        Ok(gen.content)
    }

    fn create_content_stream(&self, data: &CertificateData, _font_ref: usize, _font_bold_ref: usize) -> String {
        let mut content = String::new();
        
        // Title - Allfeat logo text
        content.push_str("BT\n");
        content.push_str("/F2 24 Tf\n");
        content.push_str("50 760 Td\n");
        content.push_str("(Allfeat.) Tj\n");
        content.push_str("ET\n");
        
        // Certificate title
        content.push_str("BT\n");
        content.push_str("/F2 24 Tf\n");
        content.push_str("0.09 0.60 0.47 rg\n"); // Teal color
        content.push_str("400 760 Td\n");
        content.push_str("(Certificate) Tj\n");
        content.push_str("ET\n");
        
        // Line under header
        content.push_str("0.09 0.60 0.47 RG\n"); // Green/teal line
        content.push_str("50 745 m\n545 745 l\nS\n");
        
        // File label and value
        content.push_str("BT\n");
        content.push_str("/F2 10 Tf\n");
        content.push_str("0.09 0.60 0.47 rg\n");
        content.push_str("50 720 Td\n");
        content.push_str("(File :) Tj\n");
        content.push_str("ET\n");
        
        content.push_str("BT\n");
        content.push_str("/F1 10 Tf\n");
        content.push_str("0 0 0 rg\n");
        content.push_str("200 720 Td\n");
        content.push_str(&format!("({}) Tj\n", escape_pdf_string(&data.asset_filename)));
        content.push_str("ET\n");
        
        // Title of work
        content.push_str("BT\n");
        content.push_str("/F2 10 Tf\n");
        content.push_str("0.09 0.60 0.47 rg\n");
        content.push_str("50 700 Td\n");
        content.push_str("(Title of the work :) Tj\n");
        content.push_str("ET\n");
        
        content.push_str("BT\n");
        content.push_str("/F1 10 Tf\n");
        content.push_str("0 0 0 rg\n");
        content.push_str("200 700 Td\n");
        content.push_str(&format!("({}) Tj\n", escape_pdf_string(&data.title)));
        content.push_str("ET\n");
        
        // Hash fields with 0x prefix and 64 characters
        let mut y_pos = 670;
        
        // hash_audio
        content.push_str("BT\n");
        content.push_str("/F2 10 Tf\n");
        content.push_str("0.09 0.60 0.47 rg\n");
        content.push_str(&format!("50 {} Td\n", y_pos));
        content.push_str("(hash_audio:) Tj\n");
        content.push_str("ET\n");
        
        content.push_str("BT\n");
        content.push_str("/F1 10 Tf\n");
        content.push_str("0 0 0 rg\n");
        content.push_str(&format!("200 {} Td\n", y_pos));
        content.push_str(&format!("(0x{}) Tj\n", generate_hash_64()));
        content.push_str("ET\n");
        y_pos -= 20;
        
        // hash_title
        content.push_str("BT\n");
        content.push_str("/F2 10 Tf\n");
        content.push_str("0.09 0.60 0.47 rg\n");
        content.push_str(&format!("50 {} Td\n", y_pos));
        content.push_str("(hash_title:) Tj\n");
        content.push_str("ET\n");
        
        content.push_str("BT\n");
        content.push_str("/F1 10 Tf\n");
        content.push_str("0 0 0 rg\n");
        content.push_str(&format!("200 {} Td\n", y_pos));
        content.push_str(&format!("(0x{}) Tj\n", generate_hash_64()));
        content.push_str("ET\n");
        y_pos -= 20;
        
        // hash_creators
        content.push_str("BT\n");
        content.push_str("/F2 10 Tf\n");
        content.push_str("0.09 0.60 0.47 rg\n");
        content.push_str(&format!("50 {} Td\n", y_pos));
        content.push_str("(hash_creators:) Tj\n");
        content.push_str("ET\n");
        
        content.push_str("BT\n");
        content.push_str("/F1 10 Tf\n");
        content.push_str("0 0 0 rg\n");
        content.push_str(&format!("200 {} Td\n", y_pos));
        content.push_str(&format!("(0x{}) Tj\n", generate_hash_64()));
        content.push_str("ET\n");
        y_pos -= 20;
        
        // secret
        content.push_str("BT\n");
        content.push_str("/F2 10 Tf\n");
        content.push_str("0.09 0.60 0.47 rg\n");
        content.push_str(&format!("50 {} Td\n", y_pos));
        content.push_str("(secret:) Tj\n");
        content.push_str("ET\n");
        
        content.push_str("BT\n");
        content.push_str("/F1 10 Tf\n");
        content.push_str("0 0 0 rg\n");
        content.push_str(&format!("200 {} Td\n", y_pos));
        content.push_str(&format!("(0x{}) Tj\n", generate_hash_64()));
        content.push_str("ET\n");
        y_pos -= 20;
        
        // hash_commitment
        content.push_str("BT\n");
        content.push_str("/F2 10 Tf\n");
        content.push_str("0.09 0.60 0.47 rg\n");
        content.push_str(&format!("50 {} Td\n", y_pos));
        content.push_str("(hash_commitment:) Tj\n");
        content.push_str("ET\n");
        
        content.push_str("BT\n");
        content.push_str("/F1 10 Tf\n");
        content.push_str("0 0 0 rg\n");
        content.push_str(&format!("200 {} Td\n", y_pos));
        content.push_str(&format!("(0x{}) Tj\n", generate_hash_64()));
        content.push_str("ET\n");
        y_pos -= 40;
        
        // Creators section title
        content.push_str("BT\n");
        content.push_str("/F2 12 Tf\n");
        content.push_str("0.09 0.60 0.47 rg\n");
        content.push_str(&format!("50 {} Td\n", y_pos));
        content.push_str("(Creators:) Tj\n");
        content.push_str("ET\n");
        y_pos -= 25;
        
        // Creators list
        for (index, creator) in data.creators.iter().enumerate() {
            // Draw creator box with subtle background
            content.push_str("0.98 0.98 0.98 rg\n"); // Very light gray
            content.push_str(&format!("50 {} 495 80 re\nf\n", y_pos - 75));
            
            // Left accent border
            content.push_str("0.09 0.60 0.47 rg\n");
            content.push_str(&format!("50 {} 3 80 re\nf\n", y_pos - 75));
            
            // Full name label
            content.push_str("BT\n");
            content.push_str("/F2 9 Tf\n");
            content.push_str("0.09 0.60 0.47 rg\n");
            content.push_str(&format!("60 {} Td\n", y_pos));
            content.push_str("(Full name :) Tj\n");
            content.push_str("ET\n");
            
            // Full name value
            content.push_str("BT\n");
            content.push_str("/F1 10 Tf\n");
            content.push_str("0 0 0 rg\n");
            content.push_str(&format!("200 {} Td\n", y_pos));
            content.push_str(&format!("({}) Tj\n", escape_pdf_string(&creator.fullname)));
            content.push_str("ET\n");
            
            // Email
            content.push_str("BT\n");
            content.push_str("/F2 9 Tf\n");
            content.push_str("0.09 0.60 0.47 rg\n");
            content.push_str(&format!("60 {} Td\n", y_pos - 18));
            content.push_str("(Email :) Tj\n");
            content.push_str("ET\n");
            
            content.push_str("BT\n");
            content.push_str("/F1 10 Tf\n");
            content.push_str("0 0 0 rg\n");
            content.push_str(&format!("200 {} Td\n", y_pos - 18));
            content.push_str(&format!("({}) Tj\n", escape_pdf_string(&creator.email)));
            content.push_str("ET\n");
            
            // Roles
            content.push_str("BT\n");
            content.push_str("/F2 9 Tf\n");
            content.push_str("0.09 0.60 0.47 rg\n");
            content.push_str(&format!("60 {} Td\n", y_pos - 36));
            content.push_str(&format!("(Role\\(s\\) :) Tj\n"));
            content.push_str("ET\n");
            
            if !creator.roles.is_empty() {
                let capitalized_roles: Vec<String> = creator.roles.iter()
                    .map(|role| {
                        let mut chars: Vec<char> = role.chars().collect();
                        if !chars.is_empty() {
                            chars[0] = chars[0].to_uppercase().next().unwrap_or(chars[0]);
                        }
                        chars.into_iter().collect()
                    })
                    .collect();
                let roles_str = capitalized_roles.join(", ");
                
                content.push_str("BT\n");
                content.push_str("/F1 10 Tf\n");
                content.push_str("0 0 0 rg\n");
                content.push_str(&format!("200 {} Td\n", y_pos - 36));
                content.push_str(&format!("({}) Tj\n", escape_pdf_string(&roles_str)));
                content.push_str("ET\n");
            }
            
            // Optional IPI
            if !creator.ipi.is_empty() {
                content.push_str("BT\n");
                content.push_str("/F2 9 Tf\n");
                content.push_str("0.09 0.60 0.47 rg\n");
                content.push_str(&format!("60 {} Td\n", y_pos - 54));
                content.push_str("(IPI :) Tj\n");
                content.push_str("ET\n");
                
                content.push_str("BT\n");
                content.push_str("/F1 10 Tf\n");
                content.push_str("0 0 0 rg\n");
                content.push_str(&format!("200 {} Td\n", y_pos - 54));
                content.push_str(&format!("({}) Tj\n", escape_pdf_string(&creator.ipi)));
                content.push_str("ET\n");
            }
            
            // Optional ISNI (align with IPI if it exists, or in its place)
            let isni_y = if !creator.ipi.is_empty() { y_pos - 54 } else { y_pos - 54 };
            if !creator.isni.is_empty() {
                content.push_str("BT\n");
                content.push_str("/F2 9 Tf\n");
                content.push_str("0.09 0.60 0.47 rg\n");
                content.push_str(&format!("320 {} Td\n", isni_y));
                content.push_str("(ISNI :) Tj\n");
                content.push_str("ET\n");
                
                content.push_str("BT\n");
                content.push_str("/F1 10 Tf\n");
                content.push_str("0 0 0 rg\n");
                content.push_str(&format!("380 {} Td\n", isni_y));
                content.push_str(&format!("({}) Tj\n", escape_pdf_string(&creator.isni)));
                content.push_str("ET\n");
            }
            
            // Add "Nom :" prefix for second creator
            if index == 1 {
                content.push_str("BT\n");
                content.push_str("/F2 9 Tf\n");
                content.push_str("0.09 0.60 0.47 rg\n");
                content.push_str(&format!("60 {} Td\n", y_pos));
                content.push_str("(Nom :) Tj\n");
                content.push_str("ET\n");
            }
            
            y_pos -= 90;
        }
        
        // Bottom separator line
        content.push_str("0.09 0.60 0.47 RG\n"); // Green/teal line
        content.push_str("50 70 m\n545 70 l\nS\n");
        
        // Footer - page number
        content.push_str("BT\n");
        content.push_str("/F1 9 Tf\n");
        content.push_str("0.4 0.4 0.4 rg\n");
        content.push_str("50 50 Td\n");
        content.push_str(&format!("({}/{}) Tj\n", data.current_page, data.total_pages));
        content.push_str("ET\n");
        
        // Footer - timestamp
        content.push_str("BT\n");
        content.push_str("/F1 9 Tf\n");
        content.push_str("0.4 0.4 0.4 rg\n");
        content.push_str("400 50 Td\n");
        content.push_str(&format!("({}) Tj\n", escape_pdf_string(&data.timestamp)));
        content.push_str("ET\n");
        
        content
    }
}

fn escape_pdf_string(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '(' => "\\(".to_string(),
            ')' => "\\)".to_string(),
            '\\' => "\\\\".to_string(),
            '\n' => "\\n".to_string(),
            '\r' => "\\r".to_string(),
            '\t' => "\\t".to_string(),
            _ => c.to_string(),
        })
        .collect()
}

// Generate a random 64-character hex string (256 bits)
fn generate_hash_64() -> String {
    let chars = "0123456789abcdef";
    let mut result = String::with_capacity(64);
    
    for _ in 0..64 {
        let random_index = (Math::random() * 16.0) as usize;
        let ch = chars.chars().nth(random_index).unwrap_or('0');
        result.push(ch);
    }
    
    result
}