----------------------------------------------------------------------------------
-- Company:
-- Engineer:
--
-- Create Date: 09/25/2024 12:21:28 PM
-- Design Name: jbh
-- Module Name: mux - Behavioral
-- Project Name: vbg
-- Target Devices:
-- Tool Versions:
-- Description:
--
-- Dependencies:
--
-- Revision:
-- Revision 0.01 - File Created
-- Additional Comments:
--
----------------------------------------------------------------------------------


library IEEE;
use IEEE.STD_LOGIC_1164.ALL;

-- Uncomment the following library declaration if using
-- arithmetic functions with Signed or Unsigned values
use IEEE.NUMERIC_STD.ALL;

-- Uncomment the following library declaration if instantiating
-- any Xilinx leaf cells in this code.
--library UNISIM;
--use UNISIM.VComponents.all;

entity mux is
    Port (
        ABorALU: in STD_LOGIC;
        Y: in unsigned (15 downto 0);
        AB: in unsigned (15 downto 0);
        C: out unsigned (15 downto 0)
    );
end mux;

architecture Behavioral of mux is
begin
    C <= AB when ABorALU = '1' else Y;
end Behavioral;
