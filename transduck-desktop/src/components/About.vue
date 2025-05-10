<template>
  <div class="about-container">
    <!-- 关于信息区 -->
    <div class="about-section">
      <div class="section-header">
        <i class="bi bi-info-circle section-icon"></i>
        <div>
          <h2>{{ $t('about.title') }}</h2>
          <p>{{ $t('about.description') }}</p>
        </div>
      </div>
      
      <div class="about-content">
        <div class="app-info">
          <div class="app-logo">
            <i class="bi bi-music-note-beamed logo-icon"></i>
            <h3>TransDuck</h3>
          </div>
          <p class="app-description">{{ $t('about.appDescription') }}</p>
        </div>
        
        <div class="links-section">
          <h4>{{ $t('about.links') }}</h4>
          <div class="link-info">
            <div class="link-item">
              <i class="bi bi-globe link-icon"></i>
              <div>
                <span class="link-label">{{ $t('about.website') }}:</span>
                <span class="link-url">https://transduck.com</span>
              </div>
            </div>
            <div class="link-item">
              <i class="bi bi-github link-icon"></i>
              <div>
                <span class="link-label">GitHub:</span>
                <span class="link-url">https://github.com/dyc87112/transduck-desktop</span>
              </div>
            </div>
          </div>
        </div>
        
        <div class="version-info">
          <p>{{ $t('about.version') }}: 0.1.0</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// 不需要打开链接的函数
</script>

<style scoped>
.about-container {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.section-header {
  display: flex;
  align-items: center;
  margin-bottom: 20px;
  gap: 15px;
}

.section-icon {
  font-size: 2rem;
  color: #0d6efd;
}

.about-section {
  padding: 25px;
  background-color: #f8f9fa;
  border-radius: 8px;
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.05);
}

.about-content {
  margin-top: 20px;
  display: flex;
  flex-direction: column;
  gap: 30px;
}

.app-info {
  text-align: center;
  padding: 20px;
  background-color: #fff;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.app-logo {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  margin-bottom: 15px;
}

.logo-icon {
  font-size: 2.5rem;
  color: #0d6efd;
}

.app-description {
  margin-top: 15px;
  color: #495057;
  line-height: 1.6;
}

.links-section {
  padding: 20px;
  background-color: #fff;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.links-section h4 {
  margin-bottom: 15px;
  color: #495057;
}

.link-info {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.link-item {
  display: flex;
  align-items: center;
  gap: 12px;
}

.link-icon {
  font-size: 1.5rem;
  color: #0d6efd;
}

.link-label {
  font-weight: 500;
  color: #495057;
  margin-right: 5px;
}

.link-url {
  color: #0d6efd;
  word-break: break-all;
}

.version-info {
  text-align: center;
  color: #6c757d;
}

.btn {
  display: inline-block;
  font-weight: 500;
  text-align: center;
  white-space: nowrap;
  vertical-align: middle;
  user-select: none;
  border: 1px solid transparent;
  padding: 0.375rem 0.75rem;
  font-size: 1rem;
  line-height: 1.5;
  border-radius: 0.25rem;
  transition: all 0.15s ease-in-out;
  cursor: pointer;
  text-decoration: none;
}

.btn i {
  margin-right: 5px;
}

.btn-primary {
  color: #fff;
  background-color: #0d6efd;
  border-color: #0d6efd;
}

.btn-primary:hover {
  background-color: #0b5ed7;
  border-color: #0a58ca;
}

.btn-dark {
  color: #fff;
  background-color: #212529;
  border-color: #212529;
}

.btn-dark:hover {
  background-color: #1c1f23;
  border-color: #1a1e21;
}
</style>