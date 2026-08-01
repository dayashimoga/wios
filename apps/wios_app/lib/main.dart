import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'dart:math' as math;
import 'pages/mesh_page.dart';
import 'pages/messages_page.dart';
import 'pages/storage_page.dart';
import 'pages/ai_page.dart';
import 'pages/compute_page.dart';
import 'pages/sensing_page.dart';
import 'pages/settings_page.dart';
import 'pages/device_sharing_page.dart';
import 'pages/location_page.dart';
import 'pages/sos_page.dart';
import 'pages/plugins_page.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  SystemChrome.setSystemUIOverlayStyle(
    const SystemUiOverlayStyle(
      statusBarColor: Colors.transparent,
      statusBarIconBrightness: Brightness.light,
    ),
  );
  runApp(const WiosApp());
}

class WiosApp extends StatelessWidget {
  const WiosApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'WIOS',
      debugShowCheckedModeBanner: false,
      themeMode: ThemeMode.system,
      theme: ThemeData(
        useMaterial3: true,
        brightness: Brightness.light,
        colorSchemeSeed: const Color(0xFF00D4AA),
        fontFamily: 'Inter',
        scaffoldBackgroundColor: const Color(0xFFF5F7FA),
        cardTheme: CardThemeData(
          color: Colors.white,
          elevation: 0,
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(16),
            side: BorderSide(color: Colors.black.withValues(alpha: 0.06)),
          ),
        ),
        appBarTheme: const AppBarTheme(
          backgroundColor: Colors.transparent,
          elevation: 0,
          centerTitle: false,
        ),
      ),
      darkTheme: ThemeData(
        useMaterial3: true,
        brightness: Brightness.dark,
        colorSchemeSeed: const Color(0xFF00D4AA),
        fontFamily: 'Inter',
        scaffoldBackgroundColor: const Color(0xFF0A0E1A),
        cardTheme: CardThemeData(
          color: const Color(0xFF141829),
          elevation: 0,
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(16),
            side: BorderSide(color: Colors.white.withValues(alpha: 0.06)),
          ),
        ),
        appBarTheme: const AppBarTheme(
          backgroundColor: Colors.transparent,
          elevation: 0,
          centerTitle: false,
        ),
      ),
      home: const WiosDashboard(),
    );
  }
}

class WiosDashboard extends StatefulWidget {
  const WiosDashboard({super.key});

  @override
  State<WiosDashboard> createState() => _WiosDashboardState();
}

class _WiosDashboardState extends State<WiosDashboard>
    with TickerProviderStateMixin {
  int _selectedIndex = 0;
  late AnimationController _pulseController;
  late AnimationController _meshAnimController;

  final List<_NavItem> _navItems = [
    _NavItem(Icons.dashboard_rounded, 'Dashboard'),
    _NavItem(Icons.hub_rounded, 'Mesh'),
    _NavItem(Icons.chat_rounded, 'Messages'),
    _NavItem(Icons.storage_rounded, 'Storage'),
    _NavItem(Icons.psychology_rounded, 'AI'),
    _NavItem(Icons.memory_rounded, 'Compute'),
    _NavItem(Icons.sensors_rounded, 'Sensing'),
    _NavItem(Icons.devices_rounded, 'Devices'),
    _NavItem(Icons.location_on_rounded, 'Location'),
    _NavItem(Icons.sos_rounded, 'SOS'),
    _NavItem(Icons.extension_rounded, 'Plugins'),
    _NavItem(Icons.settings_rounded, 'Settings'),
  ];

  @override
  void initState() {
    super.initState();
    _pulseController = AnimationController(
      vsync: this,
      duration: const Duration(seconds: 2),
    )..repeat(reverse: true);
    _meshAnimController = AnimationController(
      vsync: this,
      duration: const Duration(seconds: 8),
    )..repeat();
  }

  @override
  void dispose() {
    _pulseController.dispose();
    _meshAnimController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final isWide = MediaQuery.of(context).size.width > 800;

    return Scaffold(
      body: Row(
        children: [
          if (isWide) _buildSideNav(),
          Expanded(
            child: _buildPageContent(),
          ),
        ],
      ),
      bottomNavigationBar: isWide ? null : _buildBottomNav(),
    );
  }

  Widget _buildSideNav() {
    return Container(
      width: 72,
      decoration: BoxDecoration(
        color: const Color(0xFF0D1121),
        border: Border(
          right: BorderSide(color: Colors.white.withValues(alpha: 0.06)),
        ),
      ),
      child: Column(
        children: [
          const SizedBox(height: 16),
          _buildLogo(),
          const SizedBox(height: 24),
          Expanded(
            child: ListView.builder(
              itemCount: _navItems.length,
              itemBuilder: (context, index) {
                final isSelected = _selectedIndex == index;
                return Tooltip(
                  message: _navItems[index].label,
                  preferBelow: false,
                  child: InkWell(
                    onTap: () => setState(() => _selectedIndex = index),
                    child: Container(
                      height: 56,
                      margin: const EdgeInsets.symmetric(horizontal: 8, vertical: 2),
                      decoration: BoxDecoration(
                        borderRadius: BorderRadius.circular(12),
                        color: isSelected
                            ? const Color(0xFF00D4AA).withValues(alpha: 0.15)
                            : Colors.transparent,
                      ),
                      child: Icon(
                        _navItems[index].icon,
                        color: isSelected
                            ? const Color(0xFF00D4AA)
                            : Colors.white.withValues(alpha: 0.4),
                        size: 24,
                      ),
                    ),
                  ),
                );
              },
            ),
          ),
          Padding(
            padding: const EdgeInsets.all(12),
            child: AnimatedBuilder(
              animation: _pulseController,
              builder: (context, child) {
                return Container(
                  width: 10,
                  height: 10,
                  decoration: BoxDecoration(
                    shape: BoxShape.circle,
                    color: Color.lerp(
                      const Color(0xFF00D4AA),
                      const Color(0xFF00D4AA).withValues(alpha: 0.3),
                      _pulseController.value,
                    ),
                    boxShadow: [
                      BoxShadow(
                        color: const Color(0xFF00D4AA)
                            .withValues(alpha: 0.3 * (1 - _pulseController.value)),
                        blurRadius: 8,
                        spreadRadius: 2,
                      ),
                    ],
                  ),
                );
              },
            ),
          ),
          const SizedBox(height: 8),
        ],
      ),
    );
  }

  Widget _buildBottomNav() {
    return Container(
      decoration: BoxDecoration(
        color: const Color(0xFF0D1121),
        border: Border(
          top: BorderSide(color: Colors.white.withValues(alpha: 0.06)),
        ),
      ),
      child: BottomNavigationBar(
        currentIndex: _selectedIndex.clamp(0, 4),
        onTap: (i) => setState(() => _selectedIndex = i),
        type: BottomNavigationBarType.fixed,
        backgroundColor: Colors.transparent,
        selectedItemColor: const Color(0xFF00D4AA),
        unselectedItemColor: Colors.white.withValues(alpha: 0.4),
        showUnselectedLabels: true,
        elevation: 0,
        selectedFontSize: 11,
        unselectedFontSize: 11,
        items: _navItems.take(5).map((item) {
          return BottomNavigationBarItem(
            icon: Icon(item.icon, size: 22),
            label: item.label,
          );
        }).toList(),
      ),
    );
  }

  Widget _buildLogo() {
    return AnimatedBuilder(
      animation: _meshAnimController,
      builder: (context, child) {
        return Container(
          width: 44,
          height: 44,
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(12),
            gradient: LinearGradient(
              begin: Alignment.topLeft,
              end: Alignment.bottomRight,
              colors: [
                const Color(0xFF00D4AA),
                HSLColor.fromAHSL(1, 165 + 30 * math.sin(_meshAnimController.value * 2 * math.pi), 0.8, 0.5).toColor(),
              ],
            ),
            boxShadow: [
              BoxShadow(
                color: const Color(0xFF00D4AA).withValues(alpha: 0.3),
                blurRadius: 12,
                spreadRadius: -2,
              ),
            ],
          ),
          child: const Center(
            child: Text(
              'W',
              style: TextStyle(
                color: Colors.white,
                fontWeight: FontWeight.w800,
                fontSize: 20,
              ),
            ),
          ),
        );
      },
    );
  }

  Widget _buildPageContent() {
    switch (_selectedIndex) {
      case 0:
        return _DashboardPage(
          pulseController: _pulseController,
          meshAnimController: _meshAnimController,
        );
      case 1:
        return const MeshPage();
      case 2:
        return const MessagesPage();
      case 3:
        return const StoragePage();
      case 4:
        return const AiPage();
      case 5:
        return const ComputePage();
      case 6:
        return const SensingPage();
      case 7:
        return const DeviceSharingPage();
      case 8:
        return const LocationPage();
      case 9:
        return const SosPage();
      case 10:
        return const PluginsPage();
      case 11:
        return const SettingsPage();
      default:
        return const SizedBox.shrink();
    }
  }
}

class _DashboardPage extends StatelessWidget {
  final AnimationController pulseController;
  final AnimationController meshAnimController;

  const _DashboardPage({
    required this.pulseController,
    required this.meshAnimController,
  });

  @override
  Widget build(BuildContext context) {
    return CustomScrollView(
      slivers: [
        SliverToBoxAdapter(
          child: Padding(
            padding: const EdgeInsets.all(24),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                _buildHeader(context),
                const SizedBox(height: 24),
                _buildStatusBanner(),
                const SizedBox(height: 24),
                _buildStatsGrid(context),
                const SizedBox(height: 24),
                _buildMeshVisualization(),
                const SizedBox(height: 24),
                _buildActivitySection(),
              ],
            ),
          ),
        ),
      ],
    );
  }

  Widget _buildHeader(BuildContext context) {
    return Row(
      children: [
        Expanded(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                'WIOS Dashboard',
                style: Theme.of(context).textTheme.headlineMedium?.copyWith(
                      fontWeight: FontWeight.w700,
                      color: Colors.white,
                      letterSpacing: -0.5,
                    ),
              ),
              const SizedBox(height: 4),
              Text(
                'Wireless Intelligence Operating System',
                style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                      color: Colors.white.withValues(alpha: 0.5),
                    ),
                overflow: TextOverflow.ellipsis,
              ),
            ],
          ),
        ),
        const SizedBox(width: 12),
        _buildStatusChip('Online', const Color(0xFF00D4AA)),
        const SizedBox(width: 8),
        _buildStatusChip('Secure', const Color(0xFF6366F1)),
      ],
    );
  }

  Widget _buildStatusChip(String label, Color color) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(20),
        color: color.withValues(alpha: 0.15),
        border: Border.all(color: color.withValues(alpha: 0.3)),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Container(
            width: 6,
            height: 6,
            decoration: BoxDecoration(
              shape: BoxShape.circle,
              color: color,
            ),
          ),
          const SizedBox(width: 6),
          Text(
            label,
            style: TextStyle(
              color: color,
              fontSize: 12,
              fontWeight: FontWeight.w600,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildStatusBanner() {
    return AnimatedBuilder(
      animation: pulseController,
      builder: (context, child) {
        return Container(
          padding: const EdgeInsets.all(20),
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(16),
            gradient: LinearGradient(
              begin: Alignment.topLeft,
              end: Alignment.bottomRight,
              colors: [
                const Color(0xFF00D4AA).withValues(alpha: 0.12 + 0.03 * pulseController.value),
                const Color(0xFF6366F1).withValues(alpha: 0.08),
              ],
            ),
            border: Border.all(
              color: const Color(0xFF00D4AA).withValues(alpha: 0.15 + 0.05 * pulseController.value),
            ),
          ),
          child: Row(
            children: [
              Container(
                width: 48,
                height: 48,
                decoration: BoxDecoration(
                  borderRadius: BorderRadius.circular(12),
                  color: const Color(0xFF00D4AA).withValues(alpha: 0.2),
                ),
                child: const Icon(
                  Icons.shield_rounded,
                  color: Color(0xFF00D4AA),
                  size: 24,
                ),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    const Text(
                      'Zero Trust Security Active',
                      style: TextStyle(
                        color: Colors.white,
                        fontWeight: FontWeight.w600,
                        fontSize: 15,
                      ),
                    ),
                    const SizedBox(height: 4),
                    Text(
                      'E2EE enabled • All connections encrypted • RBAC enforced',
                      style: TextStyle(
                        color: Colors.white.withValues(alpha: 0.5),
                        fontSize: 13,
                      ),
                    ),
                  ],
                ),
              ),
            ],
          ),
        );
      },
    );
  }

  Widget _buildStatsGrid(BuildContext context) {
    final stats = [
      _StatData('Mesh Peers', '0', Icons.people_rounded, const Color(0xFF00D4AA), '+0'),
      _StatData('Messages', '0', Icons.email_rounded, const Color(0xFF6366F1), 'Queued'),
      _StatData('Storage', '0 MB', Icons.sd_storage_rounded, const Color(0xFFF59E0B), 'Encrypted'),
      _StatData('AI Models', '0', Icons.model_training_rounded, const Color(0xFFEC4899), 'Loaded'),
      _StatData('Compute', '0', Icons.developer_board_rounded, const Color(0xFF8B5CF6), 'Tasks'),
      _StatData('Uptime', '0s', Icons.timer_rounded, const Color(0xFF14B8A6), 'Active'),
    ];

    return LayoutBuilder(
      builder: (context, constraints) {
        final crossAxisCount = constraints.maxWidth > 900 ? 3 : (constraints.maxWidth > 500 ? 2 : 1);
        return GridView.builder(
          shrinkWrap: true,
          physics: const NeverScrollableScrollPhysics(),
          gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
            crossAxisCount: crossAxisCount,
            crossAxisSpacing: 16,
            mainAxisSpacing: 16,
            childAspectRatio: 2.2,
          ),
          itemCount: stats.length,
          itemBuilder: (context, index) => _buildStatCard(stats[index]),
        );
      },
    );
  }

  Widget _buildStatCard(_StatData stat) {
    return Container(
      padding: const EdgeInsets.all(20),
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(16),
        color: const Color(0xFF141829),
        border: Border.all(color: Colors.white.withValues(alpha: 0.06)),
      ),
      child: Row(
        children: [
          Container(
            width: 44,
            height: 44,
            decoration: BoxDecoration(
              borderRadius: BorderRadius.circular(12),
              color: stat.color.withValues(alpha: 0.15),
            ),
            child: Icon(stat.icon, color: stat.color, size: 22),
          ),
          const SizedBox(width: 16),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Text(
                  stat.label,
                  style: TextStyle(
                    color: Colors.white.withValues(alpha: 0.5),
                    fontSize: 12,
                    fontWeight: FontWeight.w500,
                  ),
                ),
                const SizedBox(height: 4),
                Text(
                  stat.value,
                  style: const TextStyle(
                    color: Colors.white,
                    fontSize: 24,
                    fontWeight: FontWeight.w700,
                    letterSpacing: -0.5,
                  ),
                ),
              ],
            ),
          ),
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
            decoration: BoxDecoration(
              borderRadius: BorderRadius.circular(8),
              color: stat.color.withValues(alpha: 0.1),
            ),
            child: Text(
              stat.subtitle,
              style: TextStyle(
                color: stat.color,
                fontSize: 11,
                fontWeight: FontWeight.w600,
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildMeshVisualization() {
    return Container(
      height: 280,
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(16),
        color: const Color(0xFF141829),
        border: Border.all(color: Colors.white.withValues(alpha: 0.06)),
      ),
      child: Stack(
        children: [
          // Animated mesh background
          AnimatedBuilder(
            animation: meshAnimController,
            builder: (context, child) {
              return CustomPaint(
                size: const Size(double.infinity, 280),
                painter: _MeshPainter(meshAnimController.value),
              );
            },
          ),
          Padding(
            padding: const EdgeInsets.all(20),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    const Text(
                      'Mesh Network Topology',
                      style: TextStyle(
                        color: Colors.white,
                        fontWeight: FontWeight.w600,
                        fontSize: 16,
                      ),
                    ),
                    const Spacer(),
                    Container(
                      padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 5),
                      decoration: BoxDecoration(
                        borderRadius: BorderRadius.circular(8),
                        color: const Color(0xFF00D4AA).withValues(alpha: 0.15),
                      ),
                      child: const Text(
                        'Live',
                        style: TextStyle(
                          color: Color(0xFF00D4AA),
                          fontSize: 12,
                          fontWeight: FontWeight.w600,
                        ),
                      ),
                    ),
                  ],
                ),
                const Spacer(),
                Text(
                  'No peers connected. Start the mesh to discover nearby devices.',
                  style: TextStyle(
                    color: Colors.white.withValues(alpha: 0.4),
                    fontSize: 13,
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildActivitySection() {
    return Container(
      padding: const EdgeInsets.all(20),
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(16),
        color: const Color(0xFF141829),
        border: Border.all(color: Colors.white.withValues(alpha: 0.06)),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text(
            'Recent Activity',
            style: TextStyle(
              color: Colors.white,
              fontWeight: FontWeight.w600,
              fontSize: 16,
            ),
          ),
          const SizedBox(height: 16),
          _buildActivityItem(
            Icons.power_settings_new_rounded,
            'System Started',
            'WIOS initialized successfully',
            'Just now',
            const Color(0xFF00D4AA),
          ),
          _buildActivityItem(
            Icons.security_rounded,
            'Security Initialized',
            'Zero Trust security framework active',
            'Just now',
            const Color(0xFF6366F1),
          ),
          _buildActivityItem(
            Icons.storage_rounded,
            'Storage Ready',
            'Encrypted SQLite database initialized',
            'Just now',
            const Color(0xFFF59E0B),
          ),
        ],
      ),
    );
  }

  Widget _buildActivityItem(
    IconData icon,
    String title,
    String subtitle,
    String time,
    Color color,
  ) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 12),
      child: Row(
        children: [
          Container(
            width: 36,
            height: 36,
            decoration: BoxDecoration(
              borderRadius: BorderRadius.circular(10),
              color: color.withValues(alpha: 0.15),
            ),
            child: Icon(icon, color: color, size: 18),
          ),
          const SizedBox(width: 12),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  title,
                  style: const TextStyle(
                    color: Colors.white,
                    fontSize: 13,
                    fontWeight: FontWeight.w600,
                  ),
                ),
                Text(
                  subtitle,
                  style: TextStyle(
                    color: Colors.white.withValues(alpha: 0.4),
                    fontSize: 12,
                  ),
                ),
              ],
            ),
          ),
          Text(
            time,
            style: TextStyle(
              color: Colors.white.withValues(alpha: 0.3),
              fontSize: 11,
            ),
          ),
        ],
      ),
    );
  }
}



class _MeshPainter extends CustomPainter {
  final double animValue;
  _MeshPainter(this.animValue);

  @override
  void paint(Canvas canvas, Size size) {
    final paint = Paint()
      ..style = PaintingStyle.stroke
      ..strokeWidth = 0.5;

    // Draw animated mesh lines
    final nodeCount = 12;
    final nodes = <Offset>[];
    for (int i = 0; i < nodeCount; i++) {
      final angle = (i / nodeCount) * 2 * math.pi + animValue * 2 * math.pi;
      final radius = size.width * 0.2 + math.sin(angle * 2 + animValue * 4 * math.pi) * 30;
      final x = size.width / 2 + math.cos(angle) * radius;
      final y = size.height / 2 + math.sin(angle) * radius * 0.6;
      nodes.add(Offset(x, y));
    }

    // Draw connections
    for (int i = 0; i < nodes.length; i++) {
      for (int j = i + 1; j < nodes.length; j++) {
        final dist = (nodes[i] - nodes[j]).distance;
        if (dist < size.width * 0.4) {
          final alpha = (1.0 - dist / (size.width * 0.4)) * 0.15;
          paint.color = const Color(0xFF00D4AA).withValues(alpha: alpha);
          canvas.drawLine(nodes[i], nodes[j], paint);
        }
      }
    }

    // Draw nodes
    final nodePaint = Paint()
      ..style = PaintingStyle.fill;
    for (final node in nodes) {
      nodePaint.color = const Color(0xFF00D4AA).withValues(alpha: 0.4);
      canvas.drawCircle(node, 3, nodePaint);
      nodePaint.color = const Color(0xFF00D4AA).withValues(alpha: 0.1);
      canvas.drawCircle(node, 8, nodePaint);
    }
  }

  @override
  bool shouldRepaint(covariant _MeshPainter oldDelegate) {
    return oldDelegate.animValue != animValue;
  }
}

class _NavItem {
  final IconData icon;
  final String label;
  _NavItem(this.icon, this.label);
}

class _StatData {
  final String label;
  final String value;
  final IconData icon;
  final Color color;
  final String subtitle;
  _StatData(this.label, this.value, this.icon, this.color, this.subtitle);
}
