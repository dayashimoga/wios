import 'package:flutter/material.dart';
import 'dart:math' as math;

/// Auth page — login, registration, and MFA verification.
class AuthPage extends StatefulWidget {
  final VoidCallback onAuthenticated;
  const AuthPage({super.key, required this.onAuthenticated});

  @override
  State<AuthPage> createState() => _AuthPageState();
}

class _AuthPageState extends State<AuthPage> with SingleTickerProviderStateMixin {
  late AnimationController _bgAnim;
  bool _isLogin = true;
  bool _showMfa = false;
  final _emailCtrl = TextEditingController();
  final _passCtrl = TextEditingController();
  final _mfaCtrl = TextEditingController();
  bool _obscure = true;

  @override
  void initState() {
    super.initState();
    _bgAnim = AnimationController(vsync: this, duration: const Duration(seconds: 10))..repeat();
  }

  @override
  void dispose() {
    _bgAnim.dispose();
    _emailCtrl.dispose();
    _passCtrl.dispose();
    _mfaCtrl.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0A0E1A),
      body: Stack(
        children: [
          AnimatedBuilder(
            animation: _bgAnim,
            builder: (context, _) => CustomPaint(
              size: MediaQuery.of(context).size,
              painter: _AuthBgPainter(_bgAnim.value),
            ),
          ),
          Center(
            child: SingleChildScrollView(
              padding: const EdgeInsets.all(24),
              child: ConstrainedBox(
                constraints: const BoxConstraints(maxWidth: 400),
                child: _showMfa ? _buildMfaForm() : _buildAuthForm(),
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildAuthForm() {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        // Logo
        Container(
          width: 64, height: 64,
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(16),
            gradient: const LinearGradient(
              colors: [Color(0xFF00D4AA), Color(0xFF6366F1)],
            ),
          ),
          child: const Center(
            child: Text('W', style: TextStyle(color: Colors.white, fontSize: 28, fontWeight: FontWeight.w800)),
          ),
        ),
        const SizedBox(height: 24),
        Text(_isLogin ? 'Welcome Back' : 'Create Account',
            style: const TextStyle(color: Colors.white, fontSize: 28, fontWeight: FontWeight.w700)),
        const SizedBox(height: 8),
        Text(_isLogin ? 'Sign in to your WIOS node' : 'Set up your secure identity',
            style: TextStyle(color: Colors.white.withValues(alpha: 0.5), fontSize: 14)),
        const SizedBox(height: 32),
        // Form card
        Container(
          padding: const EdgeInsets.all(24),
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(16),
            color: const Color(0xFF141829),
            border: Border.all(color: Colors.white.withValues(alpha: 0.08)),
          ),
          child: Column(
            children: [
              _field(_emailCtrl, 'Node ID or Email', Icons.person_rounded),
              const SizedBox(height: 16),
              _field(_passCtrl, 'Master Password', Icons.lock_rounded, obscure: _obscure, suffixIcon: IconButton(
                icon: Icon(_obscure ? Icons.visibility_off_rounded : Icons.visibility_rounded,
                    color: Colors.white.withValues(alpha: 0.4), size: 20),
                onPressed: () => setState(() => _obscure = !_obscure),
              )),
              if (!_isLogin) ...[
                const SizedBox(height: 16),
                _field(TextEditingController(), 'Confirm Password', Icons.lock_rounded, obscure: true),
              ],
              const SizedBox(height: 24),
              SizedBox(
                width: double.infinity,
                height: 48,
                child: FilledButton(
                  onPressed: () {
                    if (_isLogin) {
                      setState(() => _showMfa = true);
                    } else {
                      widget.onAuthenticated();
                    }
                  },
                  style: FilledButton.styleFrom(
                    backgroundColor: const Color(0xFF00D4AA),
                    foregroundColor: Colors.black,
                    shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
                  ),
                  child: Text(_isLogin ? 'Sign In' : 'Create Node Identity',
                      style: const TextStyle(fontWeight: FontWeight.w700, fontSize: 15)),
                ),
              ),
              const SizedBox(height: 16),
              Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Text(_isLogin ? "Don't have an identity? " : 'Already have an identity? ',
                      style: TextStyle(color: Colors.white.withValues(alpha: 0.5), fontSize: 13)),
                  GestureDetector(
                    onTap: () => setState(() => _isLogin = !_isLogin),
                    child: Text(_isLogin ? 'Create one' : 'Sign in',
                        style: const TextStyle(color: Color(0xFF00D4AA), fontSize: 13, fontWeight: FontWeight.w600)),
                  ),
                ],
              ),
            ],
          ),
        ),
        const SizedBox(height: 16),
        Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const Icon(Icons.shield_rounded, color: Color(0xFF00D4AA), size: 14),
            const SizedBox(width: 6),
            Text('E2EE • Argon2 • Ed25519',
                style: TextStyle(color: Colors.white.withValues(alpha: 0.3), fontSize: 11)),
          ],
        ),
      ],
    );
  }

  Widget _buildMfaForm() {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        Container(
          width: 56, height: 56,
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(14),
            color: const Color(0xFF6366F1).withValues(alpha: 0.2),
          ),
          child: const Icon(Icons.security_rounded, color: Color(0xFF6366F1), size: 28),
        ),
        const SizedBox(height: 20),
        const Text('Two-Factor Auth', style: TextStyle(color: Colors.white, fontSize: 24, fontWeight: FontWeight.w700)),
        const SizedBox(height: 8),
        Text('Enter your TOTP code', style: TextStyle(color: Colors.white.withValues(alpha: 0.5), fontSize: 14)),
        const SizedBox(height: 24),
        Container(
          padding: const EdgeInsets.all(24),
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(16),
            color: const Color(0xFF141829),
            border: Border.all(color: Colors.white.withValues(alpha: 0.08)),
          ),
          child: Column(
            children: [
              _field(_mfaCtrl, '6-digit code', Icons.pin_rounded),
              const SizedBox(height: 20),
              SizedBox(
                width: double.infinity, height: 48,
                child: FilledButton(
                  onPressed: widget.onAuthenticated,
                  style: FilledButton.styleFrom(
                    backgroundColor: const Color(0xFF6366F1),
                    shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
                  ),
                  child: const Text('Verify', style: TextStyle(fontWeight: FontWeight.w700, fontSize: 15)),
                ),
              ),
              const SizedBox(height: 12),
              TextButton(
                onPressed: () => setState(() => _showMfa = false),
                child: Text('Use recovery code', style: TextStyle(color: Colors.white.withValues(alpha: 0.5), fontSize: 13)),
              ),
            ],
          ),
        ),
      ],
    );
  }

  Widget _field(TextEditingController ctrl, String hint, IconData icon, {bool obscure = false, Widget? suffixIcon}) {
    return TextField(
      controller: ctrl,
      obscureText: obscure,
      style: const TextStyle(color: Colors.white, fontSize: 14),
      decoration: InputDecoration(
        hintText: hint,
        hintStyle: TextStyle(color: Colors.white.withValues(alpha: 0.3)),
        prefixIcon: Icon(icon, color: Colors.white.withValues(alpha: 0.4), size: 20),
        suffixIcon: suffixIcon,
        filled: true,
        fillColor: Colors.white.withValues(alpha: 0.04),
        border: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: BorderSide(color: Colors.white.withValues(alpha: 0.1)),
        ),
        enabledBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: BorderSide(color: Colors.white.withValues(alpha: 0.08)),
        ),
        focusedBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: const BorderSide(color: Color(0xFF00D4AA)),
        ),
      ),
    );
  }
}

class _AuthBgPainter extends CustomPainter {
  final double t;
  _AuthBgPainter(this.t);

  @override
  void paint(Canvas canvas, Size size) {
    final paint = Paint()..style = PaintingStyle.fill;
    for (int i = 0; i < 3; i++) {
      final cx = size.width * (0.2 + 0.3 * i) + math.sin(t * 2 * math.pi + i) * 40;
      final cy = size.height * (0.3 + 0.2 * i) + math.cos(t * 2 * math.pi + i) * 30;
      paint.shader = RadialGradient(
        colors: [
          const Color(0xFF00D4AA).withValues(alpha: 0.06),
          Colors.transparent,
        ],
      ).createShader(Rect.fromCircle(center: Offset(cx, cy), radius: 200));
      canvas.drawCircle(Offset(cx, cy), 200, paint);
    }
  }

  @override
  bool shouldRepaint(covariant _AuthBgPainter old) => old.t != t;
}
