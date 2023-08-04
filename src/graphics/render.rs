use wgpu::SurfaceError;

use super::wgpu_object::WgpuObject;

pub fn render(wobj: &mut WgpuObject) -> Result<(), SurfaceError> {
    let output = wobj.surface.get_current_texture()?;
    let view = output
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = wobj
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("commandencoder"),
        });
    {
        let color_attachment = match WgpuObject::SAMPLE_COUNT {
            1 => wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: true,
                },
            },
            _ => wgpu::RenderPassColorAttachment {
                view: &wobj.msaa_buffer,
                resolve_target: Some(&view),
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: false,
                },
            },
        };

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("RenderPass"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&wobj.pipeline);
        render_pass.set_bind_group(0, &wobj.cam_bind_group, &[]);
        render_pass.set_vertex_buffer(0, wobj.vertex_buffer.slice(..));
        render_pass.set_index_buffer(wobj.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..wobj.index_buffer_size, 0, 0..1);
    }

    wobj.queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
}